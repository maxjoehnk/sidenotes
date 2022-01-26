use crate::providers::github::GithubConfig;
use crate::providers::ProviderConfig;
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn github_settings() -> SettingsBuilder<GithubConfig> {
    SettingsBuilder::new("Github")
        // .add_field(SettingsRow::new("Query", GithubConfig::query).multiline())
        .add_field(ProviderSettingsRow::new("Token", GithubConfig::token).secret())
}

pub fn view() -> impl Widget<GithubConfig> {
    github_settings().build_view()
}

pub fn edit() -> impl Widget<GithubConfig> {
    github_settings().build_edit()
}

impl Prism<ProviderConfig, GithubConfig> for ProviderConfigPrism {
    fn get(&self, data: &ProviderConfig) -> Option<GithubConfig> {
        if let ProviderConfig::Github(config) = data {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut ProviderConfig, inner: GithubConfig) {
        *data = ProviderConfig::Github(inner);
    }
}
