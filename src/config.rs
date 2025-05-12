use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub general: GeneralConfig,
    pub filter: FilterConfig,
}

#[derive(Deserialize, Debug)]
pub struct GeneralConfig {
    pub game_path: String,
    pub staging_path: String,
}

#[derive(Deserialize, Debug)]
pub struct FilterConfig {
    pub filter_is_whitelist: bool,
    pub list: Vec<String>,
}

pub fn parse_config(raw: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(raw)
}

/// Helper function to load a config from the current working directory.
pub fn load_config_from_wd() -> Option<Config> {
    let cfg_path: PathBuf = PathBuf::from("./seedbox.toml");
    if let Ok(cfg_raw) = std::fs::read_to_string(&cfg_path) {
        parse_config(&cfg_raw).ok()
    } else {
        None
    }
}
