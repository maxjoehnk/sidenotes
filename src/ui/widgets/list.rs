use druid::theme::PLACEHOLDER_COLOR;
use druid::widget::*;
use druid::{
    Color, Command, FontDescriptor, FontFamily, FontWeight, Insets, Target, UnitPoint, Widget,
};

use crate::models::*;
use crate::ui::commands;
use crate::ui::lazy_icon::*;
use crate::ui::theme::{CARD_COLOR, STATUS_COLOR};
use crate::ui::widgets::meeting::meeting_builder;

thread_local! {
    static MENU_UP_ICON: LazyIcon = LazyIcon::new(|| {
        include_str!("../../../assets/icons/menu-up.svg").load()
    });
    static MENU_DOWN_ICON: LazyIcon = LazyIcon::new(|| {
        include_str!("../../../assets/icons/menu-down.svg").load()
    });
}

pub fn list_builder() -> impl Widget<AppState> {
    let list = List::new(provider_builder).lens(AppState::providers());
    let list_view = Flex::column()
        .with_child(meeting_builder().lens(AppState::next_appointment))
        .with_child(list);

    Scroll::new(list_view).vertical()
}

fn provider_builder() -> impl Widget<TodoProvider> {
    let title = Either::new(
        |item: &TodoProvider, _env: &_| item.collapsed,
        todo_title_builder().with_text_color(PLACEHOLDER_COLOR),
        todo_title_builder(),
    )
    .align_horizontal(UnitPoint::CENTER);
    let expand_icon = Either::new(
        |item: &TodoProvider, _env: &_| item.collapsed,
        MENU_UP_ICON.to_svg(),
        MENU_DOWN_ICON.to_svg(),
    );
    let header = Flex::row()
        .with_flex_child(title, 1.)
        .with_child(expand_icon)
        .on_click(|ctx: _, provider: &mut _, _: &_| {
            ctx.submit_command(Command::new(
                commands::TOGGLE_PROVIDER,
                provider.clone(),
                Target::Auto,
            ))
        });
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

fn todo_title_builder() -> Label<TodoProvider> {
    let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(18.0)
        .with_weight(FontWeight::BOLD);
    Label::new(|item: &TodoProvider, _env: &_| format!("{} ({})", item.name, item.items.len()))
        .with_font(font)
}

fn todo_builder() -> impl Widget<Todo> {
    let with_state = Label::new(|todo: &Todo, _env: &_| todo.state.clone().unwrap_or_default())
        .with_text_color(Color::BLACK)
        .padding(2.0)
        .background(STATUS_COLOR)
        .rounded(2.0);
    let with_state = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(title_builder())
        .with_spacer(4.0)
        .with_child(with_state);
    let without_state = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(title_builder());

    let state = Either::new(
        |todo: &Todo, _: &_| todo.state.is_some(),
        with_state,
        without_state,
    );

    state
        .padding(4.0)
        .background(CARD_COLOR)
        .rounded(2.0)
        .padding(Insets::uniform_xy(0., 2.))
        .expand_width()
        .on_click(|event_ctx, todo: &mut Todo, _: &_| {
            event_ctx.submit_command(Command::new(
                commands::OPEN_TODO,
                todo.clone(),
                Target::Auto,
            ))
        })
}

fn title_builder() -> impl Widget<Todo> {
    Label::new(|item: &Todo, _env: &_| item.title.clone())
        .with_line_break_mode(LineBreaking::WordWrap)
}
