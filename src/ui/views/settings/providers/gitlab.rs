use crate::providers::gitlab::GitlabConfig;
use crate::providers::{ProviderConfig, ProviderConfigEntry, ProviderSettings};
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn gitlab_settings() -> impl ProviderSettingsBuilder<GitlabConfig> {
    SettingsBuilder::new("Gitlab")
        .add_field(ProviderSettingsRow::new("URL", GitlabConfig::url))
        .add_field(ProviderSettingsRow::new("Token", GitlabConfig::token).secret())
        .add_flag_field(ProviderSettingsFlagRow::new(
            "Show Drafts",
            GitlabConfig::show_drafts,
        ))
}

pub fn view() -> impl Widget<(GitlabConfig, ProviderSettings)> {
    gitlab_settings().build_view()
}

pub fn edit() -> impl Widget<(GitlabConfig, ProviderSettings)> {
    gitlab_settings().build_edit()
}

impl Prism<ProviderConfigEntry, (GitlabConfig, ProviderSettings)> for ProviderConfigPrism {
    fn get(&self, entry: &ProviderConfigEntry) -> Option<(GitlabConfig, ProviderSettings)> {
        if let ProviderConfig::Gitlab(config) = &entry.provider {
            Some((config.clone(), entry.settings.clone()))
        } else {
            None
        }
    }

    fn put(&self, config: &mut ProviderConfigEntry, inner: (GitlabConfig, ProviderSettings)) {
        config.provider = ProviderConfig::Gitlab(inner.0);
        config.settings = inner.1;
    }
}
