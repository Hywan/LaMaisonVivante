use confy;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    default::Default,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub username: String,
    pub password: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
        }
    }
}

pub fn get_path() -> Result<PathBuf, &'static str> {
    let name = "kia";
    let project = ProjectDirs::from("rs", "", name)
        .ok_or_else(|| "Failed to find the configuration project directory.")?;

    let directory = project
        .config_dir()
        .to_str()
        .ok_or_else(|| "Failed to find the configuration directory.")?;

    let path: PathBuf = [directory, &format!("{}.toml", name)].iter().collect();

    Ok(path)
}

pub fn load(path: impl AsRef<Path>) -> Result<Configuration, confy::ConfyError> {
    confy::load_path(path)
}
