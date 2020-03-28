use crate::config::Config;
use crate::utils::*;
use crate::{confirm, InstallError};
use std::fs::{copy, create_dir_all};
use std::path::PathBuf;

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

    run!("timedatectl", "set-ntp", "true")
        .desc("enable ntp")
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

pub fn bootloader_in_chroot(_: &Config) -> Result<(), InstallError> {
    run!("pacman", "--noconfirm", "-S", "grub", "efibootmgr")
        .desc("install grub")
        .run()?;
    run!(
        "grub-install",
        "--debug",
        "--target=x86_64-efi",
        "--efi-directory=/efi",
        "--bootloader-id=GRUB"
    )
    .desc("Install bootloader (Grub) in UEFI mode")
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

    run!("mkfs.fat", "-F32", &config.installer.boot_disk)
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
pub fn setup_reboot_user_system(config: &Config) -> Result<(), InstallError> {
    let wd = "/mnt/root/installer";
    create_dir_all(wd)?;
    copy(
        &config.path,
        &[wd, "config.yaml"].iter().collect::<PathBuf>(),
    )?;

    copy(
        &std::env::current_exe()?,
        &[wd, "archinstaller"].iter().collect::<PathBuf>(),
    )?;
    set_file(
        "/mnt/root/continue_install.sh",
        "/root/installer/archinstaller stage user_system --config /root/installer/config.yaml",
    )?;

    run!("chmod", "+x", "/mnt/root/continue_install.sh").run()?;

    Ok(())
}
pub fn reboot(_: &Config) -> Result<(), InstallError> {
    run!("reboot", "-h", "now").desc("rebooting").run()
}
