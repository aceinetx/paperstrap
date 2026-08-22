use crate::{PaperGlobalConfig, ServerProperties, util};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::{fs, io, io::Write};

#[derive(Deserialize)]
pub struct PaperstrapConfig {
    pub build_path: PathBuf,
    pub version: String,
    pub build: usize,
    pub hash: String,
    pub server_properties: ServerProperties,
    pub paper_global_config: PaperGlobalConfig,
}

impl PaperstrapConfig {
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
        println!("downloading paper... ");
        _ = io::stdout().flush();
        match util::download(&self.get_url(), &self.get_paper_jar_path()) {
            Ok(_) => {
                println!("ok");
                Ok(())
            }
            Err(e) => {
                eprintln!("{:?}", e);
                Err(e)
            }
        }
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
}
