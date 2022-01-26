use crate::providers::confluence::ConfluenceConfig;
use crate::providers::ProviderConfig;
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn confluence_settings() -> SettingsBuilder<ConfluenceConfig> {
    SettingsBuilder::new("Confluence")
        .add_field(ProviderSettingsRow::new("URL", ConfluenceConfig::url))
        .add_field(ProviderSettingsRow::new(
            "Username",
            ConfluenceConfig::username,
        ))
        .add_field(ProviderSettingsRow::new("Password", ConfluenceConfig::password).secret())
}

pub fn view() -> impl Widget<ConfluenceConfig> {
    confluence_settings().build_view()
}

pub fn edit() -> impl Widget<ConfluenceConfig> {
    confluence_settings().build_edit()
}

impl Prism<ProviderConfig, ConfluenceConfig> for ProviderConfigPrism {
    fn get(&self, data: &ProviderConfig) -> Option<ConfluenceConfig> {
        if let ProviderConfig::Confluence(config) = data {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut ProviderConfig, inner: ConfluenceConfig) {
        *data = ProviderConfig::Confluence(inner);
    }
}
