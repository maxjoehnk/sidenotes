use crate::providers::upsource::UpsourceConfig;
use crate::providers::ProviderConfig;
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

fn upsource_settings() -> SettingsBuilder<UpsourceConfig> {
    SettingsBuilder::new("Upsource")
        .add_field(ProviderSettingsRow::new("URL", UpsourceConfig::url))
        .add_field(ProviderSettingsRow::new("Query", UpsourceConfig::query).multiline())
        .add_field(ProviderSettingsRow::new("Token", UpsourceConfig::token).secret())
}

pub fn view() -> impl Widget<UpsourceConfig> {
    upsource_settings().build_view()
}

pub fn edit() -> impl Widget<UpsourceConfig> {
    upsource_settings().build_edit()
}

impl Prism<ProviderConfig, UpsourceConfig> for ProviderConfigPrism {
    fn get(&self, data: &ProviderConfig) -> Option<UpsourceConfig> {
        if let ProviderConfig::Upsource(config) = data {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut ProviderConfig, inner: UpsourceConfig) {
        *data = ProviderConfig::Upsource(inner);
    }
}
