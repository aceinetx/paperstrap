use crate::config::*;
use crate::modrinth;
use crate::util;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DownloadModrinthPluginError {
    #[error("fetch error: {0}")]
    Fetch(#[from] modrinth::FetchPluginError),
    #[error("find matching version error: {0}")]
    FindMatchingVersion(#[from] modrinth::FindMatchingVersionError),
    #[error("download error: {0}")]
    Download(#[from] util::DownloadError),
}

#[derive(Debug, Error)]
pub enum DownloadPluginsError {
    #[error("download modrinth plugin error: {0}")]
    DownloadModrinthPlugin(#[from] DownloadModrinthPluginError),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("download error: {0}")]
    Download(#[from] util::DownloadError),
    #[error("plugin is missing `source` attribute")]
    MissingSource(),
    #[error("unknown plugin source: {0}")]
    UnknownSource(String),
}

impl PaperstrapConfig {
    pub fn get_plugin_path_from_name(&self, name: &str) -> PathBuf {
        self.build_path.join("plugins").join(format!("{name}.jar"))
    }

    fn download_modrinth_plugin(
        &self,
        name: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), DownloadModrinthPluginError> {
        let version = config.get("version");
        let game_version = &config["game_version"];
        let version_type = &config["channel"];

        let versions = modrinth::fetch_plugin(name)?;
        let meta = modrinth::find_matching_version(
            &versions,
            version.as_ref().map(|v| v.as_str()),
            game_version,
            version_type,
        )?;

        let file = &meta.files[0];
        let path = self.get_plugin_path_from_name(name);
        util::download(&file.url, &path)?;

        Ok(())
    }

    fn download_local_plugin(
        &self,
        name: &str,
        config: &HashMap<String, String>,
    ) -> io::Result<()> {
        let local_path = &config["path"];
        let path = self.get_plugin_path_from_name(name);

        _ = fs::remove_file(&path);

        symlink::symlink_dir(local_path, &path)
    }

    fn download_url_plugin(
        &self,
        name: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), util::DownloadError> {
        let url = &config["url"];
        let path = self.get_plugin_path_from_name(name);

        util::download(url, &path)
    }

    pub fn download_plugins(&self) -> Result<(), DownloadPluginsError> {
        for (name, config) in self.plugins.iter() {
            let source = match config.get("source") {
                Some(v) => v,
                None => {
                    return Err(DownloadPluginsError::MissingSource());
                }
            };

            println!("installing plugin {name}...");
            match source.as_str() {
                "modrinth" => {
                    self.download_modrinth_plugin(name, config)?;
                }
                "local" => {
                    self.download_local_plugin(name, config)?;
                }
                "url" => {
                    self.download_url_plugin(name, config)?;
                }
                other => {
                    return Err(DownloadPluginsError::UnknownSource(other.to_string()));
                }
            }
        }
        Ok(())
    }
}
