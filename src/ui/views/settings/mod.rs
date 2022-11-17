use druid::widget::*;
use druid::{Command, Target, Widget};

use crate::models::{AppState, Navigation};
use crate::ui::commands;
use crate::ui::widgets::{button_builder, header_builder};

pub mod calendar;
pub mod global;
pub mod providers;
mod widgets;

pub fn settings_builder() -> impl Widget<AppState> {
    let header = header_builder("Settings");

    let global_settings =
        button_builder("Global").on_click(|event_ctx, state: &mut AppState, _: &_| {
            event_ctx.submit_command(Command::new(
                commands::NAVIGATE,
                Navigation::GlobalSettings(state.config.clone()),
                Target::Auto,
            ))
        });

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
        .with_child(global_settings)
        .with_child(provider_settings)
        .with_child(calendar_settings)
}
