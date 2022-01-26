use druid::widget::*;
use druid::{Command, Target, Widget};

use self::meeting::meeting_builder;
use self::provider_item::provider_builder;
use crate::models::*;
use crate::ui::commands;
use crate::ui::widgets::button_builder;

mod meeting;
mod provider_item;
mod timer;
mod todo_item;

pub fn list_builder() -> impl Widget<AppState> {
    let list = List::new(provider_builder).lens(AppState::providers());
    let list_view = Flex::column()
        .with_child(meeting_builder().lens(AppState::next_appointment))
        .with_child(list);
    let list_view = Scroll::new(list_view).vertical().expand_height();
    let settings_btn = button_builder("Settings").on_click(|ctx: _, _: _, _: &_| {
        ctx.submit_command(Command::new(
            commands::NAVIGATE,
            Navigation::Settings,
            Target::Auto,
        ))
    });

    Flex::column()
        .with_flex_child(list_view, 1.0)
        .with_child(settings_btn)
        .must_fill_main_axis(true)
}
