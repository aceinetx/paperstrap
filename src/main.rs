use std::{fs, io};

use paperstrap::{PaperstrapConfig, util};

fn init_project() -> Result<(), io::Error> {
    if !util::is_dir_empty_excluding_dotfiles(std::env::current_dir()?)? {
        let ans = util::get_input("The current directory is not empty, are you sure you want to create a project here? [Y/n] ").to_lowercase();
        let confirmed = matches!(ans.as_str(), "" | "y" | "yes");
        assert!(confirmed);
    }

    fs::write("paperstrap.ncl", include_bytes!("../assets/default.ncl"))?;
    fs::write(".gitignore", include_bytes!("../assets/project-gitignore"))?;

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
        Action::Build => {
            match read_config().unwrap().build() {
                Ok(_) => println!("server built in build/"),
                Err(e) => println!("error building the server: {}", e),
            };
        }
        Action::DownloadPlugins => {
            match read_config().unwrap().download_plugins() {
                Ok(_) => println!("all plugins downloaded"),
                Err(e) => println!("error downloading plugins: {}", e),
            };
        }
        Action::Init => match init_project() {
            Ok(_) => println!("initialized a project in the current directory"),
            Err(e) => println!("failed to initialize a project: {}", e),
        },
    }
}
