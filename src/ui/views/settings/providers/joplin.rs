use crate::providers::joplin::JoplinConfig;
use crate::providers::ProviderConfig;
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn joplin_settings() -> SettingsBuilder<JoplinConfig> {
    SettingsBuilder::new("Joplin")
        .add_field(ProviderSettingsRow::new("Token", JoplinConfig::token).secret())
}

pub fn view() -> impl Widget<JoplinConfig> {
    joplin_settings().build_view()
}

pub fn edit() -> impl Widget<JoplinConfig> {
    joplin_settings().build_edit()
}

impl Prism<ProviderConfig, JoplinConfig> for ProviderConfigPrism {
    fn get(&self, data: &ProviderConfig) -> Option<JoplinConfig> {
        if let ProviderConfig::Joplin(config) = data {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut ProviderConfig, inner: JoplinConfig) {
        *data = ProviderConfig::Joplin(inner);
    }
}
