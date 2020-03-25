use ansi_term::Colour;
use dialoguer::Confirmation;
use thiserror::Error;

mod run;

mod installer;
use installer::install;

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("user declined to continue")]
    Decline,
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

fn main() {
    note("Starting Archlinux install");
    match install() {
        Ok(_) | Err(InstallError::Decline) => {}
        Err(err) => {
            error(&format!("installation failed: {}", err));
            std::process::exit(1);
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
