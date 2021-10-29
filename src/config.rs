use std::path::PathBuf;

use directories_next::ProjectDirs;
use serde::Deserialize;

use crate::providers::ProviderConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "provider")]
    pub providers: Vec<ProviderConfig>,
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
    let workspace = PathBuf::from("settings.toml");
    let xdg_home = ProjectDirs::from("me", "maxjoehnk", "sidenotes")
        .unwrap()
        .config_dir()
        .join("settings.toml");
    let file: String;

    if workspace.exists() {
        file = std::fs::read_to_string(workspace)?;
    } else {
        file = std::fs::read_to_string(xdg_home)?;
    }

    let config = toml::from_str(&file)?;

    Ok(config)
}
