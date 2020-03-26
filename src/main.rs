use crate::config::load_config;
use crate::utils::*;
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
    #[error("invalid task {0}")]
    InvalidTask(String),
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
    Task {
        #[structopt(short, long)]
        config: PathBuf,
        name: String,
    },
    ListTasks,
    DefaultConfig,
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();

    match opt {
        Opt::Install { config } => {
            let config = load_config(config)?;
            let first_stage = &config.stages.first_stage;
            let tasks = config.stages.map.get(first_stage).unwrap();
            for task in tasks {
                match task(&config) {
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
                    match task(&config) {
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
        Opt::Task { name: task, config } => {
            let config = load_config(config)?;
            if let Some(task) = tasks::TASKS.get(&task) {
                match task(&config) {
                    Ok(_) | Err(InstallError::Decline) => {}
                    Err(err) => {
                        error(&format!("failed: {}", err));
                        std::process::exit(1);
                    }
                }
            } else {
                error(&format!("there is no task {}", task));
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
