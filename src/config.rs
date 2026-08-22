use crate::{PaperGlobalConfig, ServerProperties, modrinth, util};
use nickel_lang::Context;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{io, io::Write};

#[derive(Deserialize, Debug)]
pub struct PaperstrapConfig {
    pub build_path: PathBuf,
    pub version: String,
    pub build: usize,
    pub hash: String,
    pub server_properties: ServerProperties,
    pub paper_global_config: PaperGlobalConfig,
    pub plugins: HashMap<String, HashMap<String, String>>,
}

impl PaperstrapConfig {
    pub fn compile_config(config: &str) -> Option<PaperstrapConfig> {
        let std = include_str!("../assets/std.ncl").to_owned();

        let source = std + " in " + config;

        let mut context = Context::new();

        print!("evaluating... ");
        _ = io::stdout().flush();

        let value = match context.eval_deep(&source) {
            Ok(v) => v,
            Err(_) => {
                println!("fail");
                return None;
            }
        };
        println!("ok");

        print!("converting to json... ");
        let json_str = match context.expr_to_json(&value) {
            Ok(v) => v,
            Err(e) => {
                println!("{:#?}", e);
                return None;
            }
        };
        println!("ok");

        print!("deserializing... ");
        let cfg: PaperstrapConfig = match serde_json::from_str(&json_str) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        println!("ok");

        dbg!(&cfg);

        Some(cfg)
    }

    pub fn initialize(&self) -> io::Result<()> {
        if let Err(e) = fs::create_dir(&self.build_path)
            && e.kind() != io::ErrorKind::AlreadyExists
        {
            return Err(e);
        }

        Ok(())
    }

    fn get_url(&self) -> String {
        format!(
            "https://fill-data.papermc.io/v1/objects/{}/paper-{}-{}.jar",
            self.hash, self.version, self.build
        )
    }

    fn get_paper_jar_path(&self) -> PathBuf {
        self.build_path.join("paper.jar")
    }

    fn download_paper(&self) -> Result<(), Box<dyn Error>> {
        util::download(&self.get_url(), &self.get_paper_jar_path())
    }

    pub fn download_paper_verify(&self) -> Result<(), String> {
        let dest = self.get_paper_jar_path();
        if !fs::exists(&dest).unwrap() {
            self.download_paper().map_err(|e| e.to_string())?;
        }

        print!("verifying paper server hash... ");
        _ = io::stdout().flush();
        let actual_hash = util::hash_file_sha256(&dest).unwrap();
        if actual_hash != self.hash {
            return Err(format!(
                "Server jar hash differs, expected: {}, actual: {}",
                self.hash, actual_hash
            ));
        }
        println!("ok");

        Ok(())
    }

    pub fn add_startup_scripts(&self) {
        println!("adding startup scripts...");
        for (path, contents) in HashMap::<PathBuf, &[u8; _]>::from([(
            self.build_path.join("start.sh"),
            include_bytes!("../assets/start.sh"),
        )]) {
            print!("{}... ", path.display());
            _ = io::stdout().flush();

            _ = fs::write(&path, contents);

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
    }

    pub fn add_eula(&self) {
        let path = self.build_path.join("eula.txt");
        if fs::exists(&path).unwrap() {
            return;
        }

        let ans = util::get_input(
            "Do you agree to the minecraft EULA? https://aka.ms/MinecraftEULA [Y] ",
        )
        .to_lowercase();
        let agreed = matches!(ans.as_str(), "" | "y" | "yes");
        assert!(agreed, "fuh naw");

        _ = fs::write(&path, "eula=true");
    }

    pub fn add_server_properties(&self) {
        let path = self.build_path.join("server.properties");

        print!("adding server.properties... ");
        _ = io::stdout().flush();

        match fs::write(&path, self.server_properties.to_string()) {
            Err(e) => println!("{}", e),
            Ok(_) => println!("ok"),
        }
    }

    pub fn add_paper_global_config(&self) {
        let config_dir_path = self.build_path.join("config");
        _ = fs::create_dir(&config_dir_path);
        let path = config_dir_path.join("paper-global.yml");

        print!("adding config/paper-global.yml... ");
        _ = io::stdout().flush();

        match fs::write(&path, self.paper_global_config.to_string()) {
            Err(e) => println!("{}", e),
            Ok(_) => println!("ok"),
        }
    }

    fn symlink_dir(&self, name: &str) {
        let symlink_path = self.build_path.join(name);
        let actual_path = std::env::current_dir().unwrap().join(name);

        print!("symlinking {}... ", name);
        _ = io::stdout().flush();

        if !fs::exists(&actual_path).unwrap() {
            println!("{} doesn't exist", name);
            return;
        }

        if fs::exists(&symlink_path).unwrap() {
            println!("skipped (exists)");
            return;
        }

        match symlink::symlink_dir(&actual_path, symlink_path) {
            Err(e) => println!("{}", e),
            Ok(_) => println!("ok"),
        }
    }

    pub fn symlink_world(&self) {
        self.symlink_dir("world");
    }

    pub fn symlink_plugins(&self) {
        self.symlink_dir("plugins");
    }

    pub fn build(&self) {
        self.initialize().unwrap();
        self.download_paper_verify().unwrap();
        self.add_startup_scripts();
        self.add_eula();
        self.add_server_properties();
        self.add_paper_global_config();
        self.symlink_world();
        self.symlink_plugins();
    }

    fn get_plugin_path_from_name(&self, name: &str) -> PathBuf {
        self.build_path.join("plugins").join(format!("{name}.jar"))
    }

    fn download_modrinth_plugin(
        &self,
        name: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), String> {
        let version = config.get("version");
        let game_version = &config["game_version"];
        let version_type = &config["channel"];

        let versions = modrinth::fetch_plugin(name)?;
        let meta = match modrinth::find_matching_version(
            &versions,
            version.as_ref().map(|v| v.as_str()),
            &game_version,
            &version_type,
        ) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("error while fetching plugin {name}: {e}"));
            }
        };

        let file = &meta.files[0];
        let path = self.get_plugin_path_from_name(name);
        if let Err(e) = util::download(&file.url, &path) {
            return Err(format!("error while downloading plugin {name}: {e}"));
        }

        Ok(())
    }

    fn download_local_plugin(
        &self,
        name: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), String> {
        let local_path = &config["path"];
        let path = self.get_plugin_path_from_name(name);

        _ = fs::remove_file(&path);

        symlink::symlink_dir(&local_path, &path).map_err(|e| e.to_string())
    }

    fn download_url_plugin(
        &self,
        name: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), String> {
        let url = &config["url"];
        let path = self.get_plugin_path_from_name(name);

        util::download(url, &path).map_err(|e| e.to_string())
    }

    pub fn download_plugins(&self) -> Result<(), String> {
        for (name, config) in self.plugins.iter() {
            let source = match config.get("source") {
                Some(v) => v,
                None => {
                    return Err("plugin config is missing `source` attribute".into());
                }
            };

            println!("installing plugin {name}...");
            match source.as_str() {
                "modrinth" => {
                    self.download_modrinth_plugin(name, &config)?;
                }
                "local" => {
                    self.download_local_plugin(name, &config)?;
                }
                "url" => {
                    self.download_url_plugin(name, &config)?;
                }
                other => {
                    return Err(format!("unknown plugin source {other}"));
                }
            }
        }
        Ok(())
    }
}
