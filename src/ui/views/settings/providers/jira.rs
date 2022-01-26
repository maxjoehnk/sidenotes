use crate::providers::jira::JiraConfig;
use crate::providers::ProviderConfig;
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn jira_settings() -> SettingsBuilder<JiraConfig> {
    SettingsBuilder::new("Jira")
        .add_field(ProviderSettingsRow::new("URL", JiraConfig::url))
        .add_field(ProviderSettingsRow::new("Username", JiraConfig::username))
        .add_field(ProviderSettingsRow::new("Password", JiraConfig::password).secret())
        .add_field(ProviderSettingsRow::new("Query", JiraConfig::jql).multiline())
}

pub fn view() -> impl Widget<JiraConfig> {
    jira_settings().build_view()
}

pub fn edit() -> impl Widget<JiraConfig> {
    jira_settings().build_edit()
}

impl Prism<ProviderConfig, JiraConfig> for ProviderConfigPrism {
    fn get(&self, data: &ProviderConfig) -> Option<JiraConfig> {
        if let ProviderConfig::Jira(config) = data {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut ProviderConfig, inner: JiraConfig) {
        *data = ProviderConfig::Jira(inner);
    }
}
