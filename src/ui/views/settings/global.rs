use crate::config::{Config, UiConfig};
use crate::ui::commands;
use crate::ui::views::settings::widgets::SettingsRow;
use crate::ui::widgets::{button_builder, header_builder};
use druid::widget::{CrossAxisAlignment, Flex, Switch, TextBox};
use druid::{Command, LensExt, Target, Widget, WidgetExt};

pub fn global_settings_builder() -> impl Widget<Config> {
    let header = header_builder("Global");

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
    let disable_colorized_backgrounds =
        Switch::new().lens(Config::ui.then(UiConfig::disable_colorized_backgrounds));
    let disable_colorized_backgrounds = SettingsRow::new(
        "Disable Colorized Backgrounds",
        disable_colorized_backgrounds,
    );

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .with_child(header)
        .with_child(sync_interval)
        .with_child(hide_empty_providers)
        .with_child(disable_colorized_backgrounds)
        .with_child(actions_builder())
}

fn actions_builder() -> impl Widget<Config> {
    let cancel_btn = button_builder("Cancel").on_click(|event_ctx, _, _: &_| {
        event_ctx.submit_command(Command::new(commands::NAVIGATE_BACK, (), Target::Auto))
    });
    let confirm_btn = button_builder("Save").on_click(|event_ctx, _, _: &_| {
        event_ctx.submit_command(Command::new(commands::SAVE_GLOBAL_CONFIG, (), Target::Auto))
    });

    Flex::row()
        .with_flex_child(cancel_btn, 1.0)
        .with_spacer(8.)
        .with_flex_child(confirm_btn, 1.0)
        .padding(4.)
}
