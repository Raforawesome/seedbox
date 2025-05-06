use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub filter_is_whitelist: bool,
    pub filter: FilterList,
}

#[derive(Deserialize, Debug)]
pub struct FilterList {
    pub list: Vec<String>,
}

pub fn parse_config(raw: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(raw)
}
