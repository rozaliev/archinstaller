use crate::config::Config;
use crate::utils::*;
use crate::{confirm, InstallError};

pub fn download_base(config: &Config) -> Result<(), InstallError> {
    run!("pacstrap", "/mnt", "base", "linux", "linux-firmware")
        .desc("installing essential packages")
        .run()
}

pub fn base(config: &Config) -> Result<(), InstallError> {
    run!("genfstab", "-U", "/mnt")
        .desc("Generating fstab")
        .to_file("/mnt/etc/fstab")?;

    with_chroot(config, "base_in_chroot")
}

pub fn base_in_chroot(config: &Config) -> Result<(), InstallError> {
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
    Ok(())
}

pub fn bootloader(config: &Config) -> Result<(), InstallError> {
    with_chroot(config, "bootloader_in_chroot")
}

pub fn bootloader_in_chroot(config: &Config) -> Result<(), InstallError> {
    run!("pacman", "--noconfirm", "-S", "grub")
        .desc("install grub")
        .run()?;
    // for bios
    run!("grub-install", "--debug", &config.installer.install_disk)
        .desc("Install bootloader (Grub) in bios mode")
        .run()?;

    run!("grub-mkconfig", "-o", "/boot/grub/grub.cfg")
        .desc("Generating grub config")
        .run()
}

#[must_use]
pub fn prepare(config: &Config) -> Result<(), InstallError> {
    confirm("Are you connected to Internet")?;
    confirm("Do you have your disks setup?")?;

    run!("ip", "link").desc("Current network settings").run()?;

    run!("timedatectl", "set-ntp", "true")
        .desc("Updating system clock")
        .run()?;

    note("Your disks:");
    run!("fdisk", "-l").run()?;

    confirm(&format!(
        "using {:?} as boot disk and {:?} as system disk, correct?",
        config.installer.boot_disk, config.installer.system_disk,
    ))?;

    run!("mkfs.ext4", &config.installer.boot_disk)
        .desc("formatting boot disk")
        .run()?;
    run!("mkfs.ext4", &config.installer.system_disk)
        .desc("formatting system disk")
        .run()?;

    note("mounting disks");
    run!("mount", &config.installer.system_disk, "/mnt").run()?;

    run!("mkdir", "-p", "/mnt/efi").run()?;

    run!("mount", &config.installer.boot_disk, "/mnt/efi").run()?;
    Ok(())
}
