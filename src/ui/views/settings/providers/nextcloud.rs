use crate::providers::nextcloud::deck::NextcloudDeckProviderConfig;
use crate::providers::ProviderConfig;
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

fn nextcloud_settings() -> SettingsBuilder<NextcloudDeckProviderConfig> {
    SettingsBuilder::new("Nextcloud Deck")
        .add_field(ProviderSettingsRow::new(
            "Host",
            NextcloudDeckProviderConfig::host,
        ))
        .add_field(ProviderSettingsRow::new(
            "Username",
            NextcloudDeckProviderConfig::username,
        ))
        .add_field(
            ProviderSettingsRow::new("Password", NextcloudDeckProviderConfig::password).secret(),
        )
}

pub fn view() -> impl Widget<NextcloudDeckProviderConfig> {
    nextcloud_settings().build_view()
}

pub fn edit() -> impl Widget<NextcloudDeckProviderConfig> {
    nextcloud_settings().build_edit()
}

impl Prism<ProviderConfig, NextcloudDeckProviderConfig> for ProviderConfigPrism {
    fn get(&self, data: &ProviderConfig) -> Option<NextcloudDeckProviderConfig> {
        if let ProviderConfig::NextcloudDeck(config) = data {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut ProviderConfig, inner: NextcloudDeckProviderConfig) {
        *data = ProviderConfig::NextcloudDeck(inner);
    }
}
