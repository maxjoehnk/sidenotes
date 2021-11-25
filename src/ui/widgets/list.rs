use druid::im::Vector;
use druid::widget::*;
use druid::{
    Color, Command, FontDescriptor, FontFamily, FontWeight, Insets, Target, UnitPoint, Widget,
};

use crate::models::*;

use crate::ui::commands;
use crate::ui::theme::{CARD_COLOR, STATUS_COLOR};

pub fn list_builder() -> impl Widget<Vector<TodoProvider>> {
    let list = List::new(provider_builder);
    Scroll::new(list).vertical()
}

fn provider_builder() -> impl Widget<TodoProvider> {
    let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(18.0)
        .with_weight(FontWeight::BOLD);
    let header = Label::new(|item: &TodoProvider, _env: &_| item.name.clone())
        .with_font(font)
        .align_horizontal(UnitPoint::CENTER);
    let todos = List::new(todo_builder).lens(TodoProvider::items);

    Flex::column()
        .with_child(header)
        .with_child(todos)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .padding(4.0)
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
