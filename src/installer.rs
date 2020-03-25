use crate::stages::SystemInfo;
use crate::{confirm, InstallError};
use crate::{note, prompt, run};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn install() -> Result<(), InstallError> {
    let system_info = prepare()?;

    run!("pacstrap", "/mnt", "base", "linux", "linux-firmware")
        .desc("installing essential packages")
        .run()?;

    run!("genfstab", "-U", "/mnt")
        .desc("Generating fstab")
        .to_file("/mnt/etc/fstab")?;

    run!("arch-chroot", "/mnt").desc("chrooting").run()?;

    run!(
        "ln",
        "-sf",
        "/usr/share/zoneinfo/Europe/Moscow",
        "/etc/localtime"
    )
    .desc("setting timezone")
    .run()?;

    run!("hwclock", "--systohc")
        .desc("setting Hardware Clock from Software Clock")
        .run()?;

    run!("locale-gen").desc("generating locale").run()?;

    set_file("/etc/hostname", "wpc")?;

    append_to_file("/etc/hosts", "127.0.0.1\tlocalhost")?;
    append_to_file("/etc/hosts", "::1\t\tlocalhost")?;
    append_to_file("/etc/hosts", "127.0.1.1\twpc.localdomain\twpc")?;

    run!("pacman", "--noconfirm", "-S", "intel-ucode")
        .desc("install intel microcode")
        .run()?;

    install_bootloader(&system_info.boot_disk)?;

    Ok(())
}

fn install_bootloader(boot_disk: impl AsRef<Path>) -> Result<(), InstallError> {
    // for bios
    run!("grub-install", "--target=386-pc", boot_disk.as_ref())
        .desc("Install bootloader (Grub) in bios mode")
        .run()?;

    run!("grub-mkconfig", "-o", "/boot/grub/grub.cfg")
        .desc("Generating grub config")
        .run()
}

#[must_use]
fn prepare() -> Result<SystemInfo, InstallError> {
    confirm("Are you connected to Internet")?;
    confirm("Do you have your disks setup?")?;

    run!("ip", "link").desc("Current network settings").run()?;

    run!("timedatectl", "set-ntp", "true")
        .desc("Updating system clock")
        .run()?;

    note("Your disks:");
    run!("fdisk", "-l").run()?;

    let boot_disk = exists(vec!["/dev", &prompt("What's your boot disk? (ex. sda1)")?])?;

    let system_disk = exists(vec![
        "/dev",
        &prompt("What's your system disk? (ex. sda2)")?,
    ])?;

    if boot_disk == system_disk {
        return Err(InstallError::Custom(
            "boot disk and system disk must differ",
        ));
    }

    run!("mkfs.ext4", &boot_disk)
        .desc("formatting boot disk")
        .run()?;
    run!("mkfs.ext4", &system_disk)
        .desc("formatting system disk")
        .run()?;

    note("mounting disks");
    run!("mount", &system_disk, "/mnt").run()?;

    run!("mkdir", "-p", "/mnt/efi").run()?;

    run!("mount", &boot_disk, "/mnt/efi").run()?;
    Ok(SystemInfo {
        boot_disk,
        system_disk,
    })
}

#[must_use]
fn exists(path: Vec<&str>) -> Result<PathBuf, InstallError> {
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
fn append_to_file(path: impl AsRef<Path>, s: &str) -> Result<(), InstallError> {
    note(&format!("appending {} to {:?}", s, path.as_ref()));

    let mut file = OpenOptions::new().append(true).open(path)?;

    file.write_all(s.as_bytes())?;

    note("successfully appended");
    Ok(())
}

#[must_use]
fn set_file(path: impl AsRef<Path>, s: &str) -> Result<(), InstallError> {
    note(&format!("setting file {:?} to {}", path.as_ref(), s));

    let mut file = File::create(path)?;

    file.write_all(s.as_bytes())?;
    file.write_all(s.as_bytes())?;

    note("successfully set");
    Ok(())
}
