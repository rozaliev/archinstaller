use crate::config::Installer;
use crate::installer;
use crate::InstallError;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TASKS: HashMap<String, Task> = {
        let mut m = HashMap::new();
        m.insert("prepare".to_string(), installer::prepare as Task);
        m.insert("base".to_string(), installer::base);
        m.insert("bootloader".to_string(), installer::bootloader);
        m
    };
}
pub type Task = fn(&Installer) -> Result<(), InstallError>;
