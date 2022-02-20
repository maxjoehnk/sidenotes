use crate::calendar::{CalendarConfig, CalendarConfigEntry, CalendarId};
use crate::config::Config;
use crate::models::{AppState, Navigation};
use crate::ui::commands;
use crate::ui::prism::CalendarConfigPrism;
use crate::ui::widgets::{button_builder, header_builder};
use crate::Color;
use druid::widget::{CrossAxisAlignment, Flex, List, Scroll};
use druid::{Command, Data, EventCtx, Insets, Target, Widget, WidgetExt};
use druid_widget_nursery::enum_switcher::Switcher;

mod ews;

pub fn calendar_settings_builder() -> impl Widget<AppState> {
    let header = header_builder("Calendar");
    let add_btn = button_builder("Add Calendar").on_click(|ctx: _, _, _: &_| {
        ctx.submit_command(Command::new(
            commands::NAVIGATE,
            Navigation::NewCalendar,
            Target::Auto,
        ))
    });
    let calendar_list = List::new(view_calendar_builder)
        .with_spacing(8.0)
        .lens(Config::calendar_config)
        .lens(AppState::config);
    let calendar_list = Scroll::new(calendar_list).vertical();

    Flex::column()
        .must_fill_main_axis(true)
        .with_child(header)
        .with_child(add_btn)
        .with_flex_child(calendar_list, 1.0)
}

fn view_calendar_builder() -> impl Widget<CalendarConfigEntry> {
    let mut switcher = Switcher::new();
    if cfg!(feature = "ews-calendar") {
        switcher = switcher.with_variant(CalendarConfigPrism, ews::view());
    }

    switcher
        .padding(4.)
        .background(Color::rgba8(0, 0, 0, 16))
        .rounded(2.0)
        .padding(Insets::uniform_xy(0., 2.))
        .expand_width()
        .lens(CalendarConfigEntry::config)
        .on_click(
            |event_ctx: &mut EventCtx, calendar: &mut CalendarConfigEntry, _: &_| {
                event_ctx.submit_command(Command::new(
                    commands::NAVIGATE,
                    Navigation::EditCalendar((calendar.id, calendar.config.clone())),
                    Target::Auto,
                ))
            },
        )
}

pub fn edit_calendar() -> impl Widget<CalendarConfig> {
    let cancel_btn = button_builder("Cancel").on_click(|event_ctx, _, _: &_| {
        event_ctx.submit_command(Command::new(commands::NAVIGATE_BACK, (), Target::Auto))
    });
    let confirm_btn = button_builder("Save").on_click(|event_ctx, _, _: &_| {
        event_ctx.submit_command(Command::new(commands::SAVE_CALENDAR, (), Target::Auto))
    });

    let actions = Flex::row()
        .with_flex_child(cancel_btn, 1.0)
        .with_spacer(8.)
        .with_flex_child(confirm_btn, 1.0)
        .padding(4.);

    Flex::column()
        .with_child(edit_calendar_builder())
        .with_child(actions)
}

fn edit_calendar_builder() -> impl Widget<CalendarConfig> {
    let mut switcher = Switcher::new();
    if cfg!(feature = "ews-calendar") {
        switcher = switcher.with_variant(CalendarConfigPrism, ews::edit());
    }

    switcher
}

pub fn new_calendar_selector() -> impl Widget<AppState> {
    let mut selector = Flex::column();

    if cfg!(feature = "ews-calendar") {
        add_calendar::<crate::calendar::ews::EwsConfig, _>(&mut selector, "EWS");
    }

    let header = header_builder("Add Calendar");

    Flex::column()
        .must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .with_child(header)
        .with_flex_child(selector, 1.0)
}

fn add_calendar<C: Default + Into<CalendarConfig>, T: Data>(selector: &mut Flex<T>, title: &str) {
    selector.add_child(button_builder(title).on_click(|event_ctx, _, _: &_| {
        event_ctx.submit_command(Command::new(
            commands::NAVIGATE,
            Navigation::EditCalendar((CalendarId::default(), C::default().into())),
            Target::Auto,
        ))
    }));
}
