use std::{fs, io};

use paperstrap::PaperstrapConfig;

fn init_project() -> Result<(), io::Error> {
    fs::write("paperstrap.ncl", include_bytes!("../assets/default.ncl"))?;
    fs::create_dir("plugins")?;
    fs::create_dir("world")?;

    Ok(())
}

fn read_config() -> Option<PaperstrapConfig> {
    let config = match fs::read_to_string("paperstrap.ncl") {
        Ok(text) => text,
        Err(e) => {
            println!("failed to read config: {}", e);
            return None;
        }
    };

    let Some(cfg) = PaperstrapConfig::compile_config(&config) else {
        println!("failed to compile config");
        return None;
    };

    Some(cfg)
}

enum Action {
    Build,
    DownloadPlugins,
    Init,
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let action = match args.get(1).map(|f| f.as_str()) {
        None => Action::Build,
        Some("build") => Action::Build,
        Some("download-plugins") => Action::DownloadPlugins,
        Some("init") => Action::Init,
        Some(action) => {
            println!("invalid action: {}", action);
            println!("valid actions are: build | download-plugins | init");
            return;
        }
    };

    match action {
        Action::Build => read_config().unwrap().build(),
        Action::DownloadPlugins => read_config().unwrap().download_plugins().unwrap(),
        Action::Init => match init_project() {
            Ok(_) => println!("initialized a project in the current directory"),
            Err(e) => println!("failed to initialize a project: {}", e),
        },
    }
}
