use crate::providers::nextcloud::deck::NextcloudDeckProviderConfig;
use crate::providers::{ProviderConfig, ProviderConfigEntry, ProviderSettings};
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

fn nextcloud_settings() -> impl ProviderSettingsBuilder<NextcloudDeckProviderConfig> {
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

pub fn view() -> impl Widget<(NextcloudDeckProviderConfig, ProviderSettings)> {
    nextcloud_settings().build_view()
}

pub fn edit() -> impl Widget<(NextcloudDeckProviderConfig, ProviderSettings)> {
    nextcloud_settings().build_edit()
}

impl Prism<ProviderConfigEntry, (NextcloudDeckProviderConfig, ProviderSettings)>
    for ProviderConfigPrism
{
    fn get(
        &self,
        entry: &ProviderConfigEntry,
    ) -> Option<(NextcloudDeckProviderConfig, ProviderSettings)> {
        if let ProviderConfig::NextcloudDeck(config) = &entry.provider {
            Some((config.clone(), entry.settings.clone()))
        } else {
            None
        }
    }

    fn put(
        &self,
        config: &mut ProviderConfigEntry,
        inner: (NextcloudDeckProviderConfig, ProviderSettings),
    ) {
        config.provider = ProviderConfig::NextcloudDeck(inner.0);
        config.settings = inner.1;
    }
}
