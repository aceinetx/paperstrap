use pico_args::Arguments;
use std::{env, fs, io, process::Command};

use paperstrap::{config::PaperstrapConfig, manpages, util};

fn init_project() -> Result<(), io::Error> {
    if !util::is_dir_empty_excluding_dotfiles(env::current_dir()?)? {
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

fn help() {
    println!(
        r#"paperstrap - declaratively create PaperMC minecraft servers

actions:
+ init                             initialize a project in the current directory

+ build                            builds the server project

+ run                              runs the server
    options:
    + --no-build                   don't build the server project before running

+ download-plugins                 downloads plugins for the server
    options:
    + --missing                    download only missing plugins

    list of plugins can be provided:
    $ paperstrap download-plugins vault
    ... and be combined with --missing
    $ paperstrap download-plugins --missing vault coreprotect

+ man                              show manual pages"#
    );
    for manpage in manpages::MANPAGES.iter() {
        println!("    + {:<28} {}", manpage.name, manpage.description);
    }
}

fn man(name: &str) {
    for manpage in manpages::MANPAGES.iter() {
        if name == manpage.name {
            println!("{}", manpage.description);
            println!("{}", manpage.text.trim_start().trim_end());
            return;
        }
    }
    println!("no such manpage: {}", name);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Arguments::from_env();
    let command: String = match args.free_from_str() {
        Ok(v) => v,
        Err(e) => {
            help();
            return Err(e.into());
        }
    };

    match command.as_str() {
        "init" => {
            init_project()?;
        }
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
        "man" => {
            let name: String = args.free_from_str()?;
            man(&name);
        }
        "run" => {
            let mut cfg = read_config()?;

            // setup environment for nix
            if let Ok(nix_ld_paths) = env::var("NIX_LD_LIBRARY_PATH") {
                let ld_library_path = env::var("LD_LIBRARY_PATH").unwrap_or_default();
                unsafe {
                    // SAFETY: single threaded
                    env::set_var("LD_LIBRARY_PATH", nix_ld_paths + ":" + &ld_library_path);
                }
            }

            let no_build = args.contains("--no-build");
            if !no_build {
                cfg.build()?;
            }

            let mut command = Command::new("java");
            command.args(cfg.java_args);
            command.arg("-jar");
            command.arg("paper.jar");
            command.args(cfg.paper_args);

            println!(
                "running command: {} {:?}",
                command.get_program().display(),
                command.get_args()
            );

            env::set_current_dir(cfg.build_path)?;

            let status = command.spawn()?.wait()?;
            if !status.success() {
                eprintln!("server closed with an error");
            }
        }
        _ => {
            help();
        }
    }

    Ok(())
}
