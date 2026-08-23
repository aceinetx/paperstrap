use crate::{config::PaperstrapConfig, util};
use std::{fmt::Display, fs, io, io::Write, path::PathBuf};
use thiserror::Error;

#[derive(Debug)]
pub struct HashMismatchError {
    expected: String,
    actual: String,
}

impl Display for HashMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "hash mismatch: expected {}, actual {}",
            self.expected, self.actual
        )
    }
}

#[derive(Debug, Error)]
pub enum DownloadPaperError {
    #[error("download error: {0}")]
    Io(#[from] util::DownloadError),
    #[error("hash mismatch: {0}")]
    HashMismatch(HashMismatchError),
}

impl From<HashMismatchError> for DownloadPaperError {
    fn from(value: HashMismatchError) -> Self {
        Self::HashMismatch(value)
    }
}

impl PaperstrapConfig {
    fn get_url(&self) -> String {
        format!(
            "https://fill-data.papermc.io/v1/objects/{}/paper-{}-{}.jar",
            self.hash, self.version, self.build
        )
    }

    fn get_paper_jar_path(&self) -> PathBuf {
        self.build_path.join("paper.jar")
    }

    pub(super) fn download_paper(&self) -> Result<(), DownloadPaperError> {
        let dest = self.get_paper_jar_path();
        if !fs::exists(&dest).unwrap() {
            util::download(&self.get_url(), &self.get_paper_jar_path())?;
        }

        print!("verifying paper server hash... ");
        _ = io::stdout().flush();
        let actual_hash = util::hash_file_sha256(&dest).unwrap();
        if actual_hash != self.hash {
            return Err(HashMismatchError {
                expected: self.hash.clone(),
                actual: actual_hash,
            }
            .into());
        }
        println!("ok");

        Ok(())
    }
}
