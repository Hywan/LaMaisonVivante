use confy;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    default::Default,
    net,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub database_url: String,
    pub blinds_url: net::SocketAddr,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            database_url: String::new(),
            blinds_url: net::SocketAddr::V4(net::SocketAddrV4::new(
                net::Ipv4Addr::new(127, 0, 0, 1),
                1234,
            )),
        }
    }
}

pub fn get_path() -> Result<PathBuf, &'static str> {
    let name = "hub-event-automator";
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
