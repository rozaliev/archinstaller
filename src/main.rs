use crate::utils::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;
#[macro_use]
mod run;

mod config;
mod installer;
mod tasks;
mod utils;

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("user declined to continue")]
    Decline,

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("invalid file: {0}")]
    InvalidFile(String),

    #[error("{0}")]
    Custom(&'static str),

    #[error("command returned empty response")]
    EmptyResponse,
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "ArchLinux installer",
    about = "Well, installs ArchLinux, i guess...",
    author
)]
enum Opt {
    Install {
        #[structopt(short, long)]
        config: PathBuf,
    },
    Stage {
        #[structopt(short, long)]
        config: PathBuf,
        name: String,
    },
    ListTasks,
    DefaultConfig,
}

fn load_config(path: PathBuf) -> Result<config::Config, std::io::Error> {
    let mut cfg_file = File::open(&path)?;
    let mut cfg_str = String::new();
    cfg_file.read_to_string(&mut cfg_str)?;
    let config: config::Config = toml::from_str(&cfg_str)?;

    if !config.stages.map.contains_key(&config.stages.first_stage) {
        error(&format!(
            "first stage '{}' does not exist",
            config.stages.first_stage
        ));
        std::process::exit(1);
    }

    Ok(config)
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();

    match opt {
        Opt::Install { config } => {
            let config = load_config(config)?;
            let first_stage = config.stages.first_stage;
            let tasks = config.stages.map.get(&first_stage).unwrap();
            for task in tasks {
                match task(&config.installer) {
                    Ok(_) | Err(InstallError::Decline) => {}
                    Err(err) => {
                        error(&format!("failed: {}", err));
                        std::process::exit(1);
                    }
                }
            }
        }
        Opt::Stage {
            name: stage,
            config,
        } => {
            let config = load_config(config)?;
            if let Some(tasks) = config.stages.map.get(&stage) {
                for task in tasks {
                    match task(&config.installer) {
                        Ok(_) | Err(InstallError::Decline) => {}
                        Err(err) => {
                            error(&format!("failed: {}", err));
                            std::process::exit(1);
                        }
                    }
                }
            } else {
                error(&format!("there is no stage {}", stage));
                std::process::exit(1);
            }
        }
        Opt::ListTasks => {
            note("List of all tasks:");
            for name in tasks::TASKS.keys() {
                println!("{}", name);
            }
        }
        Opt::DefaultConfig => {
            println!("{}", config::Config::default().to_string());
        }
    }
    Ok(())
}
