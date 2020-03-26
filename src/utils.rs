use crate::InstallError;
use ansi_term::Colour;
use dialoguer::{Confirmation, Input};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[must_use]
pub fn exists(path: Vec<&str>) -> Result<PathBuf, InstallError> {
    let mut buf = PathBuf::new();
    for p in path {
        buf.push(p);
    }
    let p = buf.as_path();
    if p.exists() && !p.is_dir() {
        Ok(buf)
    } else {
        Err(InstallError::InvalidFile(
            p.to_str().expect("path to str faield").to_string(),
        ))
    }
}

#[must_use]
pub fn append_to_file(path: impl AsRef<Path>, s: &str) -> Result<(), InstallError> {
    note(&format!("appending {} to {:?}", s, path.as_ref()));

    let mut file = OpenOptions::new().append(true).open(path)?;

    file.write_all(s.as_bytes())?;

    note("successfully appended");
    Ok(())
}

#[must_use]
pub fn set_file(path: impl AsRef<Path>, s: &str) -> Result<(), InstallError> {
    note(&format!("setting file {:?} to {}", path.as_ref(), s));

    let mut file = File::create(path)?;

    file.write_all(s.as_bytes())?;
    file.write_all(s.as_bytes())?;

    note("successfully set");
    Ok(())
}

pub fn confirm(s: &str) -> Result<(), InstallError> {
    if Confirmation::new()
        .with_text(&Colour::Green.bold().paint(s).to_string())
        .interact()?
    {
        Ok(())
    } else {
        Err(InstallError::Decline)
    }
}

pub fn prompt(s: &str) -> Result<String, InstallError> {
    let input: String = Input::new().with_prompt(s).interact()?;
    Ok(input)
}

pub fn note(s: &str) {
    println!("{} {}", Colour::Yellow.bold().paint("NOTE:"), s);
}

pub fn error(s: &str) {
    eprintln!(
        "{} {}",
        Colour::Red.bold().paint("Error:"),
        Colour::Cyan.paint(s)
    );
}
