use crate::providers::upsource::UpsourceConfig;
use crate::providers::{ProviderConfig, ProviderConfigEntry, ProviderSettings};
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

fn upsource_settings() -> impl ProviderSettingsBuilder<UpsourceConfig> {
    SettingsBuilder::new("Upsource")
        .add_field(ProviderSettingsRow::new("URL", UpsourceConfig::url))
        .add_field(ProviderSettingsRow::new("Query", UpsourceConfig::query).multiline())
        .add_field(ProviderSettingsRow::new("Token", UpsourceConfig::token).secret())
}

pub fn view() -> impl Widget<(UpsourceConfig, ProviderSettings)> {
    upsource_settings().build_view()
}

pub fn edit() -> impl Widget<(UpsourceConfig, ProviderSettings)> {
    upsource_settings().build_edit()
}

impl Prism<ProviderConfigEntry, (UpsourceConfig, ProviderSettings)> for ProviderConfigPrism {
    fn get(&self, entry: &ProviderConfigEntry) -> Option<(UpsourceConfig, ProviderSettings)> {
        if let ProviderConfig::Upsource(config) = &entry.provider {
            Some((config.clone(), entry.settings.clone()))
        } else {
            None
        }
    }

    fn put(&self, config: &mut ProviderConfigEntry, inner: (UpsourceConfig, ProviderSettings)) {
        config.provider = ProviderConfig::Upsource(inner.0);
        config.settings = inner.1;
    }
}
