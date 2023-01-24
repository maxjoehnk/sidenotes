use crate::providers::devops::AzureDevopsConfig;
use crate::providers::{ProviderConfig, ProviderConfigEntry, ProviderSettings};
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn devops_settings() -> impl ProviderSettingsBuilder<AzureDevopsConfig> {
    SettingsBuilder::new("Azure DevOps")
        .add_field(ProviderSettingsRow::new("Token", AzureDevopsConfig::token).secret())
        .add_field(ProviderSettingsRow::new(
            "Organization",
            AzureDevopsConfig::organization,
        ))
        .add_field(ProviderSettingsRow::new(
            "Project",
            AzureDevopsConfig::project,
        ))
}

pub fn view() -> impl Widget<(AzureDevopsConfig, ProviderSettings)> {
    devops_settings().build_view()
}

pub fn edit() -> impl Widget<(AzureDevopsConfig, ProviderSettings)> {
    devops_settings().build_edit()
}

impl Prism<ProviderConfigEntry, (AzureDevopsConfig, ProviderSettings)> for ProviderConfigPrism {
    fn get(&self, entry: &ProviderConfigEntry) -> Option<(AzureDevopsConfig, ProviderSettings)> {
        if let ProviderConfig::AzureDevops(config) = &entry.provider {
            Some((config.clone(), entry.settings.clone()))
        } else {
            None
        }
    }

    fn put(&self, config: &mut ProviderConfigEntry, inner: (AzureDevopsConfig, ProviderSettings)) {
        config.provider = ProviderConfig::AzureDevops(inner.0);
        config.settings = inner.1;
    }
}
