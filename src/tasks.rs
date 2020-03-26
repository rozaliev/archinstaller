use crate::config::Config;
use crate::installer;
use crate::InstallError;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TASKS: HashMap<String, Task> = {
        let mut m = HashMap::new();
        m.insert("prepare".to_string(), installer::prepare as Task);
        m.insert("download_base".to_string(), installer::download_base);
        m.insert("base".to_string(), installer::base);
        m.insert("base_in_chroot".to_string(), installer::base_in_chroot);
        m.insert("bootloader".to_string(), installer::bootloader);
        m.insert(
            "bootloader_in_chroot".to_string(),
            installer::bootloader_in_chroot,
        );
        m
    };
}
pub type Task = fn(&Config) -> Result<(), InstallError>;
