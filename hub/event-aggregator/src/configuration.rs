use confy;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    default::Default,
    net::SocketAddr,
    num::NonZeroU64,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub addresses: Vec<SocketAddr>,
    pub refresh_rate: NonZeroU64,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            addresses: vec![],
            refresh_rate: unsafe { NonZeroU64::new_unchecked(10) },
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
