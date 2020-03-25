use ansi_term::Colour;
use dialoguer::{Confirmation, Input};
use installer::install;
use structopt::StructOpt;
use thiserror::Error;

mod run;

mod installer;
mod stages;

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
    name = "archlinuxinstaller",
    about = "Well, installs ArchLinux, i guess...",
    author
)]
enum Opt {
    Install,
    Stage { name: String },
    List,
}

fn main() {
    let opt = Opt::from_args();
    let stages = stages! {};

    match opt {
        Opt::Install => match install() {
            Ok(_) | Err(InstallError::Decline) => {}
            Err(err) => {
                error(&format!("failed: {}", err));
                std::process::exit(1);
            }
        },
        Opt::Stage { name: stage } => {
            if let Some(task) = stages.get(&stage) {
                // FIXME: this can't be empty for a stage
                let mut sys_info = stages::SystemInfo::default();

                match task(&mut sys_info) {
                    Ok(_) | Err(InstallError::Decline) => {}
                    Err(err) => {
                        error(&format!("failed: {}", err));
                        std::process::exit(1);
                    }
                }
            } else {
                error(&format!("there is no stage {}", stage));
                std::process::exit(1);
            }
        }
        Opt::List => {
            note("List of all stages:");
            for name in stages.names() {
                println!("{}", name);
            }
        }
    }
}

fn confirm(s: &str) -> Result<(), InstallError> {
    if Confirmation::new()
        .with_text(&Colour::Green.bold().paint(s).to_string())
        .interact()?
    {
        Ok(())
    } else {
        Err(InstallError::Decline)
    }
}

fn prompt(s: &str) -> Result<String, InstallError> {
    let input: String = Input::new().with_prompt(s).interact()?;
    Ok(input)
}

fn note(s: &str) {
    println!("{} {}", Colour::Yellow.bold().paint("NOTE:"), s);
}

fn error(s: &str) {
    eprintln!(
        "{} {}",
        Colour::Red.bold().paint("Error:"),
        Colour::Cyan.paint(s)
    );
}
