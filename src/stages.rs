use crate::InstallError;
use std::collections::HashMap;
use std::path::PathBuf;

#[macro_export]
macro_rules! stages {
    ($($name:ident => $fn:expr,)*) => {
        {
            let mut stages = crate::stages::Stages::new();

            $(
                stages.add(stringify!($name).to_owned(), Box::new($fn));
            )*

            stages
        }
    }
}

#[derive(Debug, Default)]
pub struct SystemInfo {
    pub boot_disk: PathBuf,
    pub system_disk: PathBuf,
}
pub type Stage = fn(&mut SystemInfo) -> Result<(), InstallError>;

pub struct Stages {
    order: Vec<String>,
    funcs: HashMap<String, Stage>,
}

impl Stages {
    pub fn new() -> Stages {
        Stages {
            order: Vec::new(),
            funcs: HashMap::new(),
        }
    }
    pub fn get(&self, name: &str) -> Option<Stage> {
        self.funcs.get(name).map(|p| p.clone())
    }

    pub fn add(&mut self, name: String, f: Stage) {
        self.order.push(name.clone());
        self.funcs.insert(name, f);
    }

    pub fn names(&self) -> std::slice::Iter<'_, String> {
        self.order.iter()
    }
}
