use crate::config::Config;
use crate::InstallError;
use std::env;
use std::fs::remove_file;
use std::path::PathBuf;
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

pub fn rust_packages(config: &Config) -> Result<(), InstallError> {
    let script_path = format!("/home/{}/rustup.sh", config.user.name);
    run!(
        "curl",
        "--proto",
        "=https",
        "--tlsv1.2",
        "-sSf",
        "https://sh.rustup.rs",
        "-o",
        &script_path
    )
    .desc("getting rustup script")
    .run()?;

    run!("chmod", "+x", &script_path).run()?;

    run!(&script_path, "--default-toolchain", "nightly", "-y")
        .desc("running rustup install script")
        .run()?;

    remove_file(&script_path)?;
    
    let path = match env::var_os("PATH") {
        Some(path) => path,
        None => "".into()
    };

    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    paths.push(["/home", &config.user.name, ".cargo/bin"].iter().collect::<PathBuf>());
    let new_path = env::join_paths(paths)?;
    env::set_var("PATH", &new_path);

    run!(
        "cargo",
        "install",
        "cargo-edit",
        "procs",
        "exa",
        "bingrep",
        "starhip"
    )
    .run()?;
    run!(
        "cargo",
        "install",
        "-f",
        "--git",
        "https://github.com/cjbassi/ytop",
        "ytop"
    )
    .run()?;
    run!(
        "cargo",
        "install",
        "-f",
        "--git",
        "https://github.com/bootandy/dust"
    )
    .run()?;
    Ok(())
}

pub fn vpn(_: &Config) -> Result<(), InstallError> {
    unimplemented!()
}
