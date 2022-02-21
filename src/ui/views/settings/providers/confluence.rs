use crate::providers::confluence::ConfluenceConfig;
use crate::providers::{ProviderConfig, ProviderConfigEntry, ProviderSettings};
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn confluence_settings() -> impl ProviderSettingsBuilder<ConfluenceConfig> {
    SettingsBuilder::new("Confluence")
        .add_field(ProviderSettingsRow::new("URL", ConfluenceConfig::url))
        .add_field(ProviderSettingsRow::new(
            "Username",
            ConfluenceConfig::username,
        ))
        .add_field(ProviderSettingsRow::new("Password", ConfluenceConfig::password).secret())
}

pub fn view() -> impl Widget<(ConfluenceConfig, ProviderSettings)> {
    confluence_settings().build_view()
}

pub fn edit() -> impl Widget<(ConfluenceConfig, ProviderSettings)> {
    confluence_settings().build_edit()
}

impl Prism<ProviderConfigEntry, (ConfluenceConfig, ProviderSettings)> for ProviderConfigPrism {
    fn get(&self, entry: &ProviderConfigEntry) -> Option<(ConfluenceConfig, ProviderSettings)> {
        if let ProviderConfig::Confluence(config) = &entry.provider {
            Some((config.clone(), entry.settings.clone()))
        } else {
            None
        }
    }

    fn put(&self, config: &mut ProviderConfigEntry, inner: (ConfluenceConfig, ProviderSettings)) {
        config.provider = ProviderConfig::Confluence(inner.0);
        config.settings = inner.1;
    }
}
