use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
pub struct VersionFileMeta {
    pub url: String,
    pub filename: String,
}

#[derive(Deserialize, Debug)]
pub struct VersionMeta {
    pub version_number: String,
    pub game_versions: Vec<String>,
    pub version_type: String,
    pub loaders: Vec<String>,
    pub files: Vec<VersionFileMeta>,
}

#[derive(Debug, Error)]
pub enum FetchPluginError {
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("deserialize error: {0}")]
    Deserialize(#[from] serde_json::Error),
}

pub fn fetch_plugin(name: &str) -> Result<Vec<VersionMeta>, FetchPluginError> {
    let url = format!("https://api.modrinth.com/v2/project/{name}/version");
    let response = reqwest::blocking::get(url)?;
    let text = response.text()?;

    let meta: Vec<VersionMeta> = serde_json::from_str(&text)?;

    Ok(meta)
}

#[derive(Debug, Error)]
pub enum FindMatchingVersionError {
    #[error("no version found")]
    NoVersionFound(),
    #[error("invalid channel: {0}, valid values are: release, beta, alpha")]
    InvalidChannel(String),
}

pub fn find_matching_version<'a>(
    versions: &'a [VersionMeta],
    version: Option<&str>,
    game_version: &str,
    version_type: &str,
) -> Result<&'a VersionMeta, FindMatchingVersionError> {
    if !["release", "beta", "alpha"].contains(&version_type) {
        return Err(FindMatchingVersionError::InvalidChannel(
            version_type.to_string(),
        ));
    }

    for meta in versions.iter() {
        // check plugin version (if specified)
        if let Some(version) = version {
            if meta.version_number == version {
                return Ok(meta);
            } else {
                continue;
            }
        }

        // check game version
        if !meta.game_versions.contains(&game_version.into()) {
            continue;
        }

        // check version type
        if meta.version_type != version_type {
            continue;
        }

        // check for loader
        if !meta.loaders.contains(&String::from("paper"))
            && !meta.loaders.contains(&String::from("bukkit"))
        {
            continue;
        }

        return Ok(meta);
    }

    Err(FindMatchingVersionError::NoVersionFound())
}
