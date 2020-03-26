use crate::stages::Stage as Task;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

lazy_static! {
    static ref TASKS: HashMap<String, Task> = {
        let mut m = HashMap::new();
        m
    };
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub installer: Installer,
}

#[derive(Serialize, Deserialize)]
pub struct Installer {
    #[serde(with = "existing_device_path_from_name")]
    pub system_disk: PathBuf,
    #[serde(with = "existing_device_path_from_name")]
    pub boot_disk: PathBuf,
}

pub type StagesMap = HashMap<String, Vec<Task>>;

#[derive(Serialize, Deserialize)]
pub struct Stages {
    first_stage: String,
    #[serde(with = "stages_map")]
    map: HashMap<String, Vec<Task>>,
}

mod stages_map {
    use super::{StagesMap, TASKS};
    use crate::stages::Stage as Task;
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
        unimplemented!()
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
