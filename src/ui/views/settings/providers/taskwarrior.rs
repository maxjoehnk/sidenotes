use crate::providers::taskwarrior::TaskwarriorConfig;
use crate::providers::ProviderConfig;
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::views::settings::widgets::*;
use druid::Widget;
use druid_widget_nursery::prism::Prism;

fn taskwarrior_settings() -> SettingsBuilder<TaskwarriorConfig> {
    SettingsBuilder::new("Taskwarrior")
        .add_field(ProviderSettingsRow::new("Query", TaskwarriorConfig::query).multiline())
}

pub fn view() -> impl Widget<TaskwarriorConfig> {
    taskwarrior_settings().build_view()
}

pub fn edit() -> impl Widget<TaskwarriorConfig> {
    taskwarrior_settings().build_edit()
}

impl Prism<ProviderConfig, TaskwarriorConfig> for ProviderConfigPrism {
    fn get(&self, data: &ProviderConfig) -> Option<TaskwarriorConfig> {
        if let ProviderConfig::Taskwarrior(config) = data {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut ProviderConfig, inner: TaskwarriorConfig) {
        *data = ProviderConfig::Taskwarrior(inner);
    }
}
