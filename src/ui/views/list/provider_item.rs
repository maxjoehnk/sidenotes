use druid::theme::PLACEHOLDER_COLOR;
use druid::widget::*;
use druid::{Command, FontDescriptor, FontFamily, FontWeight, Target, UnitPoint, Widget};

use super::todo_item::todo_builder;
use crate::models::*;
use crate::ui::commands;
use crate::ui::lazy_icon::*;
use crate::ui::widgets::*;

thread_local! {
    static MENU_UP_ICON: LazyIcon = LazyIcon::new(|| {
        include_str!("../../../../assets/icons/menu-up.svg").load()
    });
    static MENU_DOWN_ICON: LazyIcon = LazyIcon::new(|| {
        include_str!("../../../../assets/icons/menu-down.svg").load()
    });
}

pub fn provider_builder() -> impl Widget<TodoProvider> {
    let header = provider_header();
    let todos = Either::new(
        |item: &TodoProvider, _env: &_| item.collapsed,
        Flex::column(),
        List::new(todo_builder).lens(TodoProvider::items),
    );

    Flex::column()
        .with_child(header)
        .with_child(todos)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .padding(4.0)
}

fn provider_header() -> impl Widget<TodoProvider> {
    let title = Either::new(
        |item: &TodoProvider, _env: &_| item.collapsed,
        provider_title_builder().with_text_color(PLACEHOLDER_COLOR),
        provider_title_builder(),
    )
    .align_horizontal(UnitPoint::CENTER);
    let expand_icon = Either::new(
        |item: &TodoProvider, _env: &_| item.collapsed,
        MENU_UP_ICON.to_svg(),
        MENU_DOWN_ICON.to_svg(),
    );

    Flex::row()
        .with_flex_child(title, 1.)
        .with_child(expand_icon)
        .controller(ClickableArea)
        .on_click(|ctx: _, provider: &mut _, _: &_| {
            ctx.submit_command(Command::new(
                commands::TOGGLE_PROVIDER,
                provider.clone(),
                Target::Auto,
            ))
        })
}

fn provider_title_builder() -> Label<TodoProvider> {
    let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(18.0)
        .with_weight(FontWeight::BOLD);
    Label::new(|item: &TodoProvider, _env: &_| format!("{} ({})", item.name, item.items.len()))
        .with_font(font)
}
