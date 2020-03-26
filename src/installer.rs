use crate::config::Installer;
use crate::utils::*;
use crate::{confirm, InstallError};

pub fn base(_: &Installer) -> Result<(), InstallError> {
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

    Ok(())
}

pub fn bootloader(installer: &Installer) -> Result<(), InstallError> {
    // for bios
    run!("grub-install", "--target=386-pc", &installer.boot_disk)
        .desc("Install bootloader (Grub) in bios mode")
        .run()?;

    run!("grub-mkconfig", "-o", "/boot/grub/grub.cfg")
        .desc("Generating grub config")
        .run()
}

#[must_use]
pub fn prepare(installer: &Installer) -> Result<(), InstallError> {
    confirm("Are you connected to Internet")?;
    confirm("Do you have your disks setup?")?;

    run!("ip", "link").desc("Current network settings").run()?;

    run!("timedatectl", "set-ntp", "true")
        .desc("Updating system clock")
        .run()?;

    note("Your disks:");
    run!("fdisk", "-l").run()?;

    prompt(&format!(
        "using {:?} as boot disk and {:?} as system disk, correct?",
        installer.boot_disk, installer.system_disk,
    ))?;

    run!("mkfs.ext4", &installer.boot_disk)
        .desc("formatting boot disk")
        .run()?;
    run!("mkfs.ext4", &installer.system_disk)
        .desc("formatting system disk")
        .run()?;

    note("mounting disks");
    run!("mount", &installer.system_disk, "/mnt").run()?;

    run!("mkdir", "-p", "/mnt/efi").run()?;

    run!("mount", &installer.boot_disk, "/mnt/efi").run()?;
    Ok(())
}
