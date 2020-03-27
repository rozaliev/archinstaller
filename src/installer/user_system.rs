use crate::config::Config;
use crate::utils::*;
use crate::InstallError;
use std::fs::{copy, create_dir_all, remove_file};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

pub fn set_in_qemu_http_proxy(_: &Config) -> Result<(), InstallError> {
    std::env::set_var("http_proxy", "http://10.0.2.2:3128");

    Ok(())
}

pub fn systemd_network(_: &Config) -> Result<(), InstallError> {
    set_file(
        "/etc/systemd/network/MyDhcp.network",
        "[Match]
Name=en*

[Network]
DHCP=ipv4",
    )?;

    run!("systemctl", "enable", "--now", "systemd-networkd.service")
        .desc("enabling networkd")
        .run()?;
    run!("systemctl", "enable", "--now", "systemd-resolved.service")
        .desc("enabling resolved")
        .run()?;

    for _ in 0..10 {
        note("waiting for network");
        let out = run!("ip", "a").to_string()?;
        if out.contains("state UP") {
            return Ok(());
        }

        sleep(Duration::from_secs(1));
    }

    Err(InstallError::NoNetwork)
}
pub fn essential_packages(_: &Config) -> Result<(), InstallError> {
    run!("pacman", "-Sy")
        .desc("update pacman databases")
        .run()?;

    run!(
        "pacman",
        "--noconfirm",
        "-S",
        "neovim",
        "smbclient",
        "base-devel",
        "curl",
        "git",
        "openssh",
        "man"
    )
    .desc("install packages")
    .run()?;
    Ok(())
}
pub fn vga(_: &Config) -> Result<(), InstallError> {
    run!("pacman", "--noconfirm", "-S", "xf86-video-vesa")
        .desc("install packages")
        .run()
}
pub fn audio(_: &Config) -> Result<(), InstallError> {
    unimplemented!()
}

pub fn terminal_packages(_: &Config) -> Result<(), InstallError> {
    run!(
        "pacman",
        "--noconfirm",
        "-S",
        "alacritty",
        "xclip",
        "ttf-fira-code",
        "fish",
        "ranger",
        "bat",
        "ttf-fira-code",
        "hexyl",
        "rustup",
        "broot",
        "fd",
        "ripgrep"

    )
    .desc("install packages")
    .run()?;

     Ok(())
}
pub fn desktop(_: &Config) -> Result<(), InstallError> {
    run!(
        "pacman",
        "--noconfirm",
        "-S",
        "xorg-server",
        "lightdm",
        "lightdm-gtk-greeter",
        "i3-gaps",
        "i3lock",
        "rofi",
        "maim",
    )
    .desc("install packages")
    .run()?;

    // TOOD: install polybar
    run!("systemctl", "enable", "lightdm.service")
        .desc("enabling lightdm")
        .run()?;

    Ok(())
}

pub fn add_user(config: &Config) -> Result<(), InstallError> {
    run!(
        "useradd",
        "-g",
        "users",
        "--create-home",
        "--shell",
        "/usr/bin/fish",
        &config.user.name
    )
    .desc("create user")
    .run()?;

    run!("chpasswd")
        .desc("setting password to username")
        .run_with_stdin(format!("{}:{}", config.user.name, config.user.name).as_bytes())?;

    append_to_file(
        "/etc/sudoers",
        &format!("{} ALL=(ALL) ALL", config.user.name),
    )?;

    Ok(())
}
pub fn cleanup_reboot_hook(config: &Config) -> Result<(), InstallError> {
    remove_file("/root/.bashrc")?;

    Ok(())
}

pub fn generate_ssh_keys(config: &Config) -> Result<(), InstallError> {
    run!("mkdir", "-p", format!("/home/{}/.ssh", config.user.name))
        .desc("ensure that .ssh exists")
        .run()?;
    run!(
        "ssh-keygen",
        "-t",
        "rsa",
        "-b",
        "4096",
        "-C",
        &config.user.email,
        "-f",
        format!("/home/{}/.ssh/id_rsa", config.user.name),
        "-N",
        "\"\"",
    )
    .run()
}
pub fn disable_root_login(_: &Config) -> Result<(), InstallError> {
    unimplemented!()
}
pub fn power_management(_: &Config) -> Result<(), InstallError> {
    unimplemented!()
}
pub fn clock_sync(_: &Config) -> Result<(), InstallError> {
    unimplemented!()
}
pub fn firewall(_: &Config) -> Result<(), InstallError> {
    unimplemented!()
}
pub fn setup_reboot_post_install(config: &Config) -> Result<(), InstallError> {
    create_dir_all(format!("/home/{}/installer", config.user.name))?;
    copy(
        &config.path,
        &["/home", &config.user.name, "installer/config.yaml"]
            .iter()
            .collect::<PathBuf>(),
    )?;

    copy(
        &std::env::current_exe()?,
        &["/home", &config.user.name, "installer/archinstaller"]
            .iter()
            .collect::<PathBuf>(),
    )?;
    set_file(
        format!("/home/{}/continue_install.sh", config.user.name),
        "#!/bin/bash
~/installer/archinstaller stage post_install --config ~/installer/config.yaml",
    )?;

    run!(
        "chown",
        "-R",
        format!("{}:users", config.user.name),
        "/home/erz"
    )
    .desc("fix permissions")
    .run()?;
    run!(
        "chmod",
        "+x",
        &["/home", &config.user.name, "continue_install.sh"]
            .iter()
            .collect::<PathBuf>()
    )
    .desc("make installer runnable")
    .run()?;

    Ok(())
}
