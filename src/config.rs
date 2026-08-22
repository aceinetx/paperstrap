use crate::util;
use serde::Deserialize;
use std::error::Error;
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct PaperstrapConfig {
    pub build_path: PathBuf,
    pub version: String,
    pub build: usize,
    pub hash: String,
}

impl PaperstrapConfig {
    pub fn initialize(&self) -> io::Result<()> {
        if let Err(e) = std::fs::create_dir(&self.build_path)
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
        println!("downloading paper...");
        util::download(&self.get_url(), &self.get_paper_jar_path())
    }

    pub fn download_paper_verify(&self) -> Result<(), String> {
        let dest = self.get_paper_jar_path();
        if !std::fs::exists(&dest).map_err(|e| e.to_string())? {
            self.download_paper().map_err(|e| e.to_string())?;
        }

        print!("verifying paper server hash... ");
        _ = io::stdout().flush();
        let actual_hash = util::hash_file_sha256(&dest).map_err(|e| e.to_string())?;
        if actual_hash != self.hash {
            return Err(format!(
                "Server jar hash differs, expected: {}, actual: {}",
                self.hash, actual_hash
            ));
        }
        println!("ok");

        Ok(())
    }
}
