use crate::providers::github::GithubConfig;
use crate::providers::{ProviderConfig, ProviderConfigEntry, ProviderSettings};
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn github_settings() -> impl ProviderSettingsBuilder<GithubConfig> {
    SettingsBuilder::new("Github")
        // .add_field(ProviderSettingsRow::optional("Query", GithubConfig::query).multiline())
        .add_field(ProviderSettingsRow::new("Token", GithubConfig::token).secret())
}

pub fn view() -> impl Widget<(GithubConfig, ProviderSettings)> {
    github_settings().build_view()
}

pub fn edit() -> impl Widget<(GithubConfig, ProviderSettings)> {
    github_settings().build_edit()
}

impl Prism<ProviderConfigEntry, (GithubConfig, ProviderSettings)> for ProviderConfigPrism {
    fn get(&self, entry: &ProviderConfigEntry) -> Option<(GithubConfig, ProviderSettings)> {
        if let ProviderConfig::Github(config) = &entry.provider {
            Some((config.clone(), entry.settings.clone()))
        } else {
            None
        }
    }

    fn put(&self, config: &mut ProviderConfigEntry, inner: (GithubConfig, ProviderSettings)) {
        config.provider = ProviderConfig::Github(inner.0);
        config.settings = inner.1;
    }
}
