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
    #[serde(default, rename = "provider", skip_serializing_if = "Vector::is_empty")]
    pub providers: Vector<ProviderConfigEntry>,
    #[serde(default, rename = "calendar", skip_serializing_if = "Vector::is_empty")]
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
    let file_path = get_config_path();
    let file = std::fs::read_to_string(&file_path)?;

    let config = toml::from_str(&file)?;

    Ok((config, file_path))
}

pub fn save(path: &Path, config: &Config) -> anyhow::Result<()> {
    if !path.exists() {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
    }
    let config = toml::to_string(config)?;
    std::fs::write(path, config)?;

    Ok(())
}

fn get_config_path() -> PathBuf {
    let workspace = PathBuf::from("settings.toml");
    let xdg_home = ProjectDirs::from("me", "maxjoehnk", "sidenotes")
        .expect("Home directory could not be detected")
        .config_dir()
        .join("settings.toml");

    if workspace.exists() {
        workspace
    } else {
        xdg_home
    }
}

pub fn get_config_save_path() -> PathBuf {
    if let Some(path) = ProjectDirs::from("me", "maxjoehnk", "sidenotes")
        .map(|dirs| dirs.config_dir().join("settings.toml"))
    {
        path
    } else {
        PathBuf::from("settings.toml")
    }
}
