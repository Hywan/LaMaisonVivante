use crate::command::AddressWithRefreshRate;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub addresses: Vec<AddressWithRefreshRate>,
    pub database_url: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            addresses: vec![],
            database_url: String::new(),
        }
    }
}

pub fn get_path() -> Result<PathBuf, &'static str> {
    let name = "hub-event-aggregator";
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
