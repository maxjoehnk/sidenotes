use crate::providers::gitlab::GitlabConfig;
use crate::providers::ProviderConfig;
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn gitlab_settings() -> SettingsBuilder<GitlabConfig> {
    SettingsBuilder::new("Gitlab")
        .add_field(ProviderSettingsRow::new("URL", GitlabConfig::url))
        .add_field(ProviderSettingsRow::new("Token", GitlabConfig::token).secret())
}

pub fn view() -> impl Widget<GitlabConfig> {
    gitlab_settings().build_view()
}

pub fn edit() -> impl Widget<GitlabConfig> {
    gitlab_settings().build_edit()
}

impl Prism<ProviderConfig, GitlabConfig> for ProviderConfigPrism {
    fn get(&self, data: &ProviderConfig) -> Option<GitlabConfig> {
        if let ProviderConfig::Gitlab(config) = data {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut ProviderConfig, inner: GitlabConfig) {
        *data = ProviderConfig::Gitlab(inner);
    }
}
