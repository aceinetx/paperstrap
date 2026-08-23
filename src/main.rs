use pico_args::Arguments;
use std::{fs, io};

use paperstrap::{config::PaperstrapConfig, util};

fn init_project() -> Result<(), io::Error> {
    if !util::is_dir_empty_excluding_dotfiles(std::env::current_dir()?)? {
        let ans = util::get_input("The current directory is not empty, are you sure you want to create a project here? [Y/n] ").to_lowercase();
        let confirmed = matches!(ans.as_str(), "" | "y" | "yes");
        assert!(confirmed);
    }

    fs::write(
        "paperstrap.ncl",
        include_bytes!("../assets/project-paperstrap.ncl"),
    )?;
    fs::write(".gitignore", include_bytes!("../assets/project-gitignore"))?;
    fs::write(
        ".editorconfig",
        include_bytes!("../assets/project-editorconfig"),
    )?;

    println!("Initialized project in current directory");

    Ok(())
}

fn read_config() -> Result<PaperstrapConfig, Box<dyn std::error::Error>> {
    let config = fs::read_to_string("paperstrap.ncl")?;

    Ok(PaperstrapConfig::compile_config(&config)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Arguments::from_env();
    let command: String = args.free_from_str()?;

    match command.as_str() {
        "build" => {
            read_config()?.build()?;
        }
        "download-plugins" => {
            let only_missing = args.contains("--missing");

            let plugins: Vec<String> = args
                .finish()
                .into_iter()
                .map(|arg| arg.to_string_lossy().into_owned())
                .collect();

            read_config()?.download_plugins(only_missing, &plugins)?;
        }
        "init" => {
            init_project()?;
        }
        action => {
            println!("invalid action: {}", action);
            println!("valid actions are: build | download-plugins | init");
        }
    }

    Ok(())
}
