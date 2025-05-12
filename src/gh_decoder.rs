#![allow(dead_code)]
use serde::Deserialize;
use tracing::warn;

#[derive(Deserialize, Debug)]
pub struct GhModData {
    #[serde(rename = "$schema")]
    schema: String,
    mods: Vec<ModListing>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModListing {
    name: String,
    author: String,
    id: String,
    nexus: Option<u32>,
    curse: Option<u32>,
    moddrop: Option<u32>,
    github: Option<String>,
    broke_in: Option<String>,
    summary: Option<String>,
    unofficial_update: Option<UnofficialUpdate>,
}

impl ModListing {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnofficialUpdate {
    version: String,
    url: String,
}

pub fn parse_gh_data(data: &str) -> Vec<ModListing> {
    let Ok(data) = json5::from_str::<GhModData>(data) else {
        warn!("Failed to parse github mod data.");
        return Vec::new();
    };
    data.mods
}

pub async fn get_raw_gh_data() -> String {
    reqwest::get("https://raw.githubusercontent.com/Pathoschild/SmapiCompatibilityList/refs/heads/develop/data/mods.jsonc")
        .await.unwrap().text().await.unwrap()
}
