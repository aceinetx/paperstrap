use crate::config::{DownloadPaperError, DownloadPluginsError};
use crate::{PaperGlobalConfig, ServerProperties, util};
use nickel_lang::Context;
use serde::Deserialize;
use soft_canonicalize::soft_canonicalize;
use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{io, io::Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilationError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("deserialize error: {0}")]
    Deserialize(#[from] serde_json::Error),
    #[error("eval error")]
    Eval(nickel_lang::Error),
}

impl From<nickel_lang::Error> for CompilationError {
    fn from(value: nickel_lang::Error) -> Self {
        Self::Eval(value)
    }
}

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("download paper error: {0}")]
    DownloadPaper(#[from] DownloadPaperError),
    #[error("download plugins error: {0}")]
    DownloadPlugins(#[from] DownloadPluginsError),
}

#[derive(Deserialize, Debug)]
pub struct PaperstrapConfig {
    pub build_path: PathBuf,
    pub version: String,
    pub build: usize,
    pub hash: String,

    #[serde(default)]
    pub java_args: Vec<String>,

    #[serde(default)]
    pub paper_args: Vec<String>,

    pub server_properties: ServerProperties,
    pub paper_global_config: PaperGlobalConfig,
    pub plugins: HashMap<String, HashMap<String, String>>,

    #[serde(default)]
    pub world_source: Option<String>,

    #[serde(default)]
    pub symlink_files: HashMap<PathBuf, PathBuf>,

    #[serde(default)]
    pub symlink_dirs: HashMap<PathBuf, PathBuf>,
}

impl PaperstrapConfig {
    pub fn compile_config(config: &str) -> Result<PaperstrapConfig, CompilationError> {
        let std = include_str!("../../assets/std.ncl").to_owned();

        let source = std + " in " + config;

        let mut context = Context::new();

        print!("evaluating... ");
        _ = io::stdout().flush();

        let value = context.eval_deep(&source)?;
        println!("ok");

        print!("converting to json... ");
        let json_str = context.expr_to_json(&value)?;
        println!("ok");

        print!("deserializing... ");
        let mut cfg: PaperstrapConfig = serde_json::from_str(&json_str)?;
        println!("ok");

        cfg.build_path = std::path::absolute(cfg.build_path)?;

        Ok(cfg)
    }

    fn initialize(&self) -> io::Result<()> {
        if let Err(e) = fs::create_dir(&self.build_path)
            && e.kind() != io::ErrorKind::AlreadyExists
        {
            return Err(e);
        }

        Ok(())
    }

    fn add_startup_scripts(&self) -> io::Result<()> {
        println!("adding startup scripts...");
        for (path, contents) in HashMap::<PathBuf, &[u8; _]>::from([(
            self.build_path.join("start.sh"),
            include_bytes!("../../assets/start.sh"),
        )]) {
            print!("\t{}... ", path.display());
            _ = io::stdout().flush();

            fs::write(&path, contents)?;

            if let Ok(perms) = fs::metadata(&path) {
                let mut perms = perms.permissions();
                perms.set_mode(0o755);
                if let Err(e) = fs::set_permissions(&path, perms) {
                    println!("{}", e);
                } else {
                    println!("ok");
                }
            } else {
                println!("could not get metadata");
            }
        }

        Ok(())
    }

    fn add_eula(&self) -> io::Result<()> {
        let path = self.build_path.join("eula.txt");
        if fs::exists(&path)? {
            return Ok(());
        }

        let ans = util::get_input(
            "Do you agree to the minecraft EULA? https://aka.ms/MinecraftEULA [Y/n] ",
        )
        .to_lowercase();
        let agreed = matches!(ans.as_str(), "" | "y" | "yes");
        assert!(agreed, "fuh naw");

        fs::write(&path, "eula=true")?;
        Ok(())
    }

    fn add_server_properties(&self) -> io::Result<()> {
        let path = self.build_path.join("server.properties");

        print!("adding server.properties... ");
        _ = io::stdout().flush();

        fs::write(&path, self.server_properties.to_string())?;

        println!("ok");
        Ok(())
    }

    fn add_paper_global_config(&self) -> io::Result<()> {
        let config_dir_path = self.build_path.join("config");
        _ = fs::create_dir(&config_dir_path);
        let path = config_dir_path.join("paper-global.yml");

        print!("adding config/paper-global.yml... ");
        _ = io::stdout().flush();

        fs::write(&path, self.paper_global_config.to_string())?;
        println!("ok");
        Ok(())
    }

    fn symlink_dir(&self, actual_path: &str, name: &str) -> io::Result<()> {
        let actual_path = std::path::absolute(actual_path)?;
        let symlink_path = self.build_path.join(name);

        print!("symlinking {}... ", name);
        _ = io::stdout().flush();

        _ = fs::create_dir(&actual_path);
        if !fs::exists(&actual_path)? {
            println!("{} doesn't exist", name);
            return Ok(());
        }

        if fs::exists(&symlink_path)? {
            println!("skipped (exists)");
            return Ok(());
        }

        match symlink::symlink_dir(&actual_path, symlink_path) {
            Err(e) => println!("{}", e),
            Ok(_) => println!("ok"),
        }

        Ok(())
    }

    fn symlink_world(&self) -> io::Result<()> {
        self.symlink_dir(
            self.world_source.as_ref().unwrap_or(&"world".into()),
            "world",
        )
    }

    fn symlink_plugins(&self) -> io::Result<()> {
        self.symlink_dir("plugins", "plugins")
    }

    fn add_custom_symlinks(&mut self) -> io::Result<()> {
        println!("adding custom symlinks...");

        fn expand_paths(
            build_path: &Path,
            paths: &HashMap<PathBuf, PathBuf>,
        ) -> io::Result<HashMap<PathBuf, PathBuf>> {
            let mut new_paths = HashMap::new();
            for (from, to) in paths.iter() {
                // `from` must exist, use fs::canonicalize
                let from = fs::canonicalize(std::path::absolute(from)?)?;
                let to = std::path::absolute(build_path.join(to))?;
                // `to` may not exist, use soft_canonicalize::soft_canonicalize
                let to = soft_canonicalize(std::path::absolute(to)?)?;
                if !to.starts_with(&build_path) {
                    eprintln!("\t{} is outside the build directory!", to.display());
                    continue;
                }

                new_paths.insert(from, to);
            }
            Ok(new_paths)
        }

        self.symlink_files = expand_paths(&self.build_path, &self.symlink_files)?;
        self.symlink_dirs = expand_paths(&self.build_path, &self.symlink_dirs)?;

        for (from, to) in self.symlink_files.iter() {
            print!("\t- {} -> {}... ", from.display(), to.display());
            _ = io::stdout().flush();

            match symlink::symlink_file(from, to) {
                Ok(_) => println!("ok"),
                Err(e) => println!("{}", e),
            }
        }

        for (from, to) in self.symlink_dirs.iter() {
            print!("\td {} -> {}... ", from.display(), to.display());
            _ = io::stdout().flush();

            match symlink::symlink_dir(from, to) {
                Ok(_) => println!("ok"),
                Err(e) => println!("{}", e),
            }
        }

        Ok(())
    }

    pub fn build(&mut self, download_missing_plugins: bool) -> Result<(), BuildError> {
        self.initialize()?;
        self.download_paper()?;
        self.add_startup_scripts()?;
        self.add_eula()?;
        self.add_server_properties()?;
        self.add_paper_global_config()?;
        self.symlink_world()?;
        self.symlink_plugins()?;
        self.add_custom_symlinks()?;
        if download_missing_plugins {
            self.download_plugins(true, &[])?;
        }
        Ok(())
    }
}
