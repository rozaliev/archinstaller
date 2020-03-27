use crate::config::Config;
use crate::InstallError;

pub fn set_git_user(config: &Config) -> Result<(), InstallError> {
    run!(
        "git",
        "config",
        "--global",
        "user.email",
        &config.user.email
    )
    .run()?;
    run!(
        "git",
        "config",
        "--global",
        "user.name",
        &config.user.full_name
    )
    .run()?;

    Ok(())
}

pub fn desktop_packages(_: &Config) -> Result<(), InstallError> {
    run!(
        "pacman",
        "--noconfirm",
        "-S",
        "firefox",
        "transmission-gtk",
        "telegram-desktop",
        "vlc"
    )
    .desc("install packages")
    .run()?;
    run!("rustup", "default", "nightly").run()?;
    run!("cargo", "install", "cargo-edit", "procs","exa","bingrep", "starhip").run()?;
    run!("cargo", "install","-f","--git","https://github.com/cjbassi/ytop","ytop").run()?;
    run!("cargo", "install","-f","--git","https://github.com/bootandy/dust").run()?;
    Ok(())
}
pub fn codecs(_: &Config) -> Result<(), InstallError> {
    unimplemented!()
}
pub fn firefox_1password(_: &Config) -> Result<(), InstallError> {
    unimplemented!()
}
pub fn github(_: &Config) -> Result<(), InstallError> {
    unimplemented!()
}
pub fn setup_dotfiles(_: &Config) -> Result<(), InstallError> {
    unimplemented!()
}
pub fn vpn(_: &Config) -> Result<(), InstallError> {
    unimplemented!()
}
