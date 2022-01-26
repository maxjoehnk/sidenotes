use crate::config::{Config, UiConfig};
use druid::widget::*;
use druid::{Command, LensExt, Target, Widget};

use crate::models::{AppState, Navigation};
use crate::ui::commands;
use crate::ui::views::settings::widgets::SettingsRow;
use crate::ui::widgets::{button_builder, header_builder};

pub mod calendar;
pub mod providers;
mod widgets;

pub fn settings_builder() -> impl Widget<AppState> {
    let header = header_builder("Settings");

    let provider_settings = button_builder("Providers").on_click(|event_ctx, _, _: &_| {
        event_ctx.submit_command(Command::new(
            commands::NAVIGATE,
            Navigation::ProviderSettings,
            Target::Auto,
        ))
    });

    let calendar_settings = button_builder("Calendar").on_click(|event_ctx, _, _: &_| {
        event_ctx.submit_command(Command::new(
            commands::NAVIGATE,
            Navigation::CalendarSettings,
            Target::Auto,
        ))
    });

    Flex::column()
        .with_child(header)
        .with_child(global_config_builder().lens(AppState::config))
        .with_spacer(4.0)
        .with_child(provider_settings)
        .with_child(calendar_settings)
}

fn global_config_builder() -> impl Widget<Config> {
    let sync_interval = TextBox::new().lens(Config::sync_timeout.map::<_, _, String>(
        |timeout| timeout.to_string(),
        |timeout, new_value| {
            if let Ok(parsed) = new_value.parse() {
                *timeout = parsed;
            }
        },
    ));
    let sync_interval = SettingsRow::new("Sync Interval", sync_interval);
    let hide_empty_providers = Switch::new().lens(Config::ui.then(UiConfig::hide_empty_providers));
    let hide_empty_providers = SettingsRow::new("Hide Empty Providers", hide_empty_providers);

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .with_child(sync_interval)
        .with_child(hide_empty_providers)
}
