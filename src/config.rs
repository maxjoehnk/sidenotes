use serde::Deserialize;

use crate::providers::ProviderConfigEntry;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "provider")]
    pub providers: Vec<ProviderConfigEntry>,
    pub sync_timeout: u64,
    #[serde(flatten)]
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Copy, Deserialize, druid::Data)]
pub struct UiConfig {
    #[serde(default = "default_hide_empty_providers")]
    pub hide_empty_providers: bool,
}

fn default_hide_empty_providers() -> bool {
    true
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            hide_empty_providers: default_hide_empty_providers(),
        }
    }
}

pub fn load() -> anyhow::Result<Config> {
    let file = std::fs::read_to_string("settings.toml")?;

    let config = toml::from_str(&file)?;

    Ok(config)
}
