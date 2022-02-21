use crate::providers::taskwarrior::TaskwarriorConfig;
use crate::providers::{ProviderConfig, ProviderConfigEntry, ProviderSettings};
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

fn taskwarrior_settings() -> impl ProviderSettingsBuilder<TaskwarriorConfig> {
    SettingsBuilder::new("Taskwarrior")
        .add_field(ProviderSettingsRow::new("Query", TaskwarriorConfig::query).multiline())
}

pub fn view() -> impl Widget<(TaskwarriorConfig, ProviderSettings)> {
    taskwarrior_settings().build_view()
}

pub fn edit() -> impl Widget<(TaskwarriorConfig, ProviderSettings)> {
    taskwarrior_settings().build_edit()
}

impl Prism<ProviderConfigEntry, (TaskwarriorConfig, ProviderSettings)> for ProviderConfigPrism {
    fn get(&self, entry: &ProviderConfigEntry) -> Option<(TaskwarriorConfig, ProviderSettings)> {
        if let ProviderConfig::Taskwarrior(config) = &entry.provider {
            Some((config.clone(), entry.settings.clone()))
        } else {
            None
        }
    }

    fn put(&self, config: &mut ProviderConfigEntry, inner: (TaskwarriorConfig, ProviderSettings)) {
        config.provider = ProviderConfig::Taskwarrior(inner.0);
        config.settings = inner.1;
    }
}
