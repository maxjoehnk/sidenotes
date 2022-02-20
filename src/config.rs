use std::path::{Path, PathBuf};

use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::calendar::CalendarConfigEntry;
use crate::providers::ProviderConfigEntry;
use im::Vector;

#[derive(Default, Debug, Clone, Deserialize, Serialize, druid::Data, druid::Lens)]
pub struct Config {
    pub sync_timeout: u64,
    #[serde(flatten)]
    pub ui: UiConfig,
    #[serde(default, rename = "provider")]
    pub providers: Vector<ProviderConfigEntry>,
    #[serde(default, rename = "calendar")]
    pub calendar_config: Vector<CalendarConfigEntry>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, druid::Data, druid::Lens)]
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

pub fn load() -> anyhow::Result<(Config, PathBuf)> {
    let workspace = PathBuf::from("settings.toml");
    let xdg_home = ProjectDirs::from("me", "maxjoehnk", "sidenotes")
        .expect("Home directory could not be detected")
        .config_dir()
        .join("settings.toml");

    let (file, path) = if workspace.exists() {
        (std::fs::read_to_string(&workspace)?, workspace)
    } else {
        (std::fs::read_to_string(&xdg_home)?, xdg_home)
    };

    let config = toml::from_str(&file)?;

    Ok((config, path))
}

pub fn save(path: &Path, config: &Config) -> anyhow::Result<()> {
    let config = toml::to_string(config)?;
    std::fs::write(path, config)?;

    Ok(())
}
