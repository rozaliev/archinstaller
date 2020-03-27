use crate::config::Config;
use crate::tasks::TASKS;
use crate::InstallError;
use ansi_term::Colour;
use dialoguer::{Confirmation, Input};
use std::fs::{copy, create_dir_all, remove_dir_all, File, OpenOptions};
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
    file.write(b"\n")?;
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
        .show_default(true)
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

pub fn with_chroot(config: &Config, task: &str) -> Result<(), InstallError> {
    if !TASKS.contains_key(task) {
        return Err(InstallError::InvalidTask(task.to_owned()));
    }

    let wd = "/mnt/_chroot_install";
    create_dir_all(wd)?;
    copy(
        &config.path,
        &[wd, "config.yaml"].iter().collect::<PathBuf>(),
    )?;

    copy(
        &std::env::current_exe()?,
        &[wd, "archinstaller"].iter().collect::<PathBuf>(),
    )?;

    run!(
        "arch-chroot",
        "/mnt",
        "/_chroot_install/archinstaller",
        "task",
        task,
        "--config",
        "/_chroot_install/config.yaml"
    )
    .desc(&format!("chrooting for task {}", task))
    .run()?;

    remove_dir_all(wd)?;

    Ok(())
}
