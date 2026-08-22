use serde::Deserialize;

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

pub fn fetch_plugin(name: &str) -> Result<Vec<VersionMeta>, String> {
    let url = format!("https://api.modrinth.com/v2/project/{name}/version");
    let response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    let text = response.text().map_err(|e| e.to_string())?;

    let meta: Vec<VersionMeta> = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    Ok(meta)
}

pub fn find_matching_version<'a>(
    versions: &'a [VersionMeta],
    version: Option<&str>,
    game_version: &str,
    version_type: &str,
) -> Result<&'a VersionMeta, String> {
    if !["release", "beta", "alpha"].contains(&version_type) {
        return Err(format!(
            "invalid version type: {version_type}, valid types are: release, beta, alpha"
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
        if !meta.loaders.contains(&String::from("paper")) {
            continue;
        }

        return Ok(meta);
    }
    Err("no version found".into())
}
