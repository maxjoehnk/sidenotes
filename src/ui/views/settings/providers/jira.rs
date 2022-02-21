use crate::providers::jira::JiraConfig;
use crate::providers::{ProviderConfig, ProviderConfigEntry, ProviderSettings};
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn jira_settings() -> impl ProviderSettingsBuilder<JiraConfig> {
    SettingsBuilder::new("Jira")
        .add_field(ProviderSettingsRow::new("URL", JiraConfig::url))
        .add_field(ProviderSettingsRow::new("Username", JiraConfig::username))
        .add_field(ProviderSettingsRow::new("Password", JiraConfig::password).secret())
        .add_field(ProviderSettingsRow::new("Query", JiraConfig::jql).multiline())
}

pub fn view() -> impl Widget<(JiraConfig, ProviderSettings)> {
    jira_settings().build_view()
}

pub fn edit() -> impl Widget<(JiraConfig, ProviderSettings)> {
    jira_settings().build_edit()
}

impl Prism<ProviderConfigEntry, (JiraConfig, ProviderSettings)> for ProviderConfigPrism {
    fn get(&self, entry: &ProviderConfigEntry) -> Option<(JiraConfig, ProviderSettings)> {
        if let ProviderConfig::Jira(config) = &entry.provider {
            Some((config.clone(), entry.settings.clone()))
        } else {
            None
        }
    }

    fn put(&self, config: &mut ProviderConfigEntry, inner: (JiraConfig, ProviderSettings)) {
        config.provider = ProviderConfig::Jira(inner.0);
        config.settings = inner.1;
    }
}
