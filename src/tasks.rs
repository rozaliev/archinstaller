use crate::config::Config;
use crate::installer;
use crate::InstallError;
use lazy_static::lazy_static;
use std::collections::HashMap;

macro_rules! t {
    ($hm:expr, $i:ident) => {
        $hm.insert(stringify!($i).to_string(), installer::$i as Task);
    };
}

lazy_static! {
    pub static ref TASKS: HashMap<String, Task> = {
        let mut m = HashMap::new();
        // base stage
        t!(m, prepare);
        t!(m, download_base);
        t!(m, base);
        t!(m, base_in_chroot);
        t!(m, bootloader);
        t!(m, bootloader_in_chroot);
        t!(m, setup_reboot_user_system);
        t!(m, reboot);


        // user stage
        t!(m, set_in_qemu_http_proxy);
        t!(m, cleanup_reboot_hook);
        t!(m, systemd_network);
        t!(m, essential_packages);
        t!(m, vga);
        t!(m, audio);
        t!(m, desktop);
        t!(m, desktop_packages);
        t!(m, setup_dotfiles);
        t!(m, codecs);
        t!(m, terminal_packages);
        t!(m, add_user);
        t!(m, generate_ssh_keys);
        t!(m, disable_root_login);
        t!(m, power_management);
        t!(m, firewall);
        t!(m, setup_reboot_post_install);

        // post install
        t!(m, set_git_user);
        t!(m, rust_packages);
        t!(m, vpn);

        m
    };
}
pub type Task = fn(&Config) -> Result<(), InstallError>;
