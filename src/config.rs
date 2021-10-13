use serde::Deserialize;
use crate::providers::ProviderConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "provider")]
    pub providers: Vec<ProviderConfig>,
    pub sync_timeout: u64,
}

pub fn load() -> anyhow::Result<Config> {
    let file = std::fs::read_to_string("settings.toml")?;

    let config = toml::from_str(&file)?;

    Ok(config)
}
