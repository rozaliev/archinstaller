use crate::tasks::Task;
use crate::tasks::TASKS;
use crate::utils::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub installer: Installer,
    pub user: User,
    pub stages: Stages,
    #[serde(default, skip)]
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub full_name: String,
    pub email: String,
    pub hostname: String,
}
#[derive(Serialize, Deserialize)]
pub struct Installer {
    #[serde(with = "existing_device_path_from_name")]
    pub install_disk: PathBuf,
    #[serde(with = "existing_device_path_from_name")]
    pub system_disk: PathBuf,
    #[serde(with = "existing_device_path_from_name")]
    pub boot_disk: PathBuf,
}

pub type StagesMap = HashMap<String, Vec<Task>>;

#[derive(Serialize, Deserialize)]
pub struct Stages {
    pub first_stage: String,
    #[serde(with = "stages_map")]
    pub map: HashMap<String, Vec<Task>>,
}

impl Default for Config {
    fn default() -> Config {
        let mut m = HashMap::new();
        let s1 = vec![TASKS["prepare"], TASKS["base"]];
        m.insert("my_stage1".to_string(), s1);
        let s2 = vec![TASKS["bootloader"]];
        m.insert("my_stage2".to_string(), s2);
        Config {
            installer: Installer {
                system_disk: "/dev/sdXn".into(),
                boot_disk: "/dev/sdXn".into(),
                install_disk: "/dev/sdX".into(),
            },
            user: User {
                name: "your_login".to_string(),
                full_name: "Full Name".to_string(),
                email: "my@email.com".to_string(),
                hostname: "myhost".to_string(),
            },
            stages: Stages {
                first_stage: "my_first_stage".into(),
                map: m,
            },
            path: PathBuf::default(),
        }
    }
}

impl Config {
    pub fn to_string(&self) -> String {
        toml::to_string(self).unwrap()
    }
}

pub fn load_config(path: PathBuf) -> Result<Config, std::io::Error> {
    let mut cfg_file = File::open(&path)?;
    let mut cfg_str = String::new();
    cfg_file.read_to_string(&mut cfg_str)?;
    let mut config: Config = toml::from_str(&cfg_str)?;

    if !config.stages.map.contains_key(&config.stages.first_stage) {
        error(&format!(
            "first stage '{}' does not exist",
            config.stages.first_stage
        ));
        std::process::exit(1);
    }

    config.path = path;

    Ok(config)
}

mod stages_map {
    use super::{StagesMap, TASKS};
    use crate::tasks::Task;
    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    struct Wrapper(Task);

    pub fn deserialize<'de, D>(deserializer: D) -> Result<StagesMap, D::Error>
    where
        D: Deserializer<'de>,
    {
        let tm = HashMap::<String, Vec<Wrapper>>::deserialize(deserializer)?;
        Ok(tm
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|Wrapper(i)| i).collect()))
            .collect())
    }

    pub fn serialize<S>(m: &StagesMap, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hm: HashMap<_, Vec<_>> = m
            .iter()
            .map(|(k, v)| (k, v.iter().map(|v| Wrapper(*v)).collect()))
            .collect();

        hm.serialize(serializer)
    }

    impl<'de> Deserialize<'de> for Wrapper {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            match TASKS.get(&s) {
                Some(task) => Ok(Wrapper(task.clone())),
                None => Err(serde::de::Error::custom(format!("there is no task {}", s))),
            }
        }
    }
    impl Serialize for Wrapper {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if let Some((k, _)) = TASKS
                .iter()
                // this might not work
                .find(|(k, v)| (**v) as *const fn() == self.0 as *const fn())
            {
                String::serialize(k, serializer)
            } else {
                Err(serde::ser::Error::custom("failed to serialize task name"))
            }
        }
    }
}

mod existing_device_path_from_name {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::path::PathBuf;

    pub fn serialize<S>(path: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let filename = path
            .file_name()
            .map(|os| os.to_str())
            .flatten()
            .ok_or_else(|| serde::ser::Error::custom("invalid filename"))?;
        serializer.serialize_str(&filename)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dev_name = String::deserialize(deserializer)?;
        let mut pb = PathBuf::new();
        pb.push("/dev");
        pb.push(&dev_name);
        let p = pb.as_path();
        if p.exists() && !p.is_dir() {
            Ok(pb)
        } else {
            Err(serde::de::Error::custom(format!(
                "device {} doesn't exist",
                dev_name
            )))
        }
    }
}
