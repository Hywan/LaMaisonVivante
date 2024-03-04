use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Configuration {
    pub openweathermap_api_key: String,
    pub thing_port: Option<u16>,
}

pub fn get_path() -> Result<PathBuf, &'static str> {
    let name = "weather";
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
