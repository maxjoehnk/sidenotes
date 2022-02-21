use crate::providers::joplin::JoplinConfig;
use crate::providers::{ProviderConfig, ProviderConfigEntry, ProviderSettings};
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn joplin_settings() -> impl ProviderSettingsBuilder<JoplinConfig> {
    SettingsBuilder::new("Joplin")
        .add_field(ProviderSettingsRow::new("Token", JoplinConfig::token).secret())
}

pub fn view() -> impl Widget<(JoplinConfig, ProviderSettings)> {
    joplin_settings().build_view()
}

pub fn edit() -> impl Widget<(JoplinConfig, ProviderSettings)> {
    joplin_settings().build_edit()
}

impl Prism<ProviderConfigEntry, (JoplinConfig, ProviderSettings)> for ProviderConfigPrism {
    fn get(&self, entry: &ProviderConfigEntry) -> Option<(JoplinConfig, ProviderSettings)> {
        if let ProviderConfig::Joplin(config) = &entry.provider {
            Some((config.clone(), entry.settings.clone()))
        } else {
            None
        }
    }

    fn put(&self, config: &mut ProviderConfigEntry, inner: (JoplinConfig, ProviderSettings)) {
        config.provider = ProviderConfig::Joplin(inner.0);
        config.settings = inner.1;
    }
}
