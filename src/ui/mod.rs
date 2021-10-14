use druid::{Env, Event, EventCtx, FontDescriptor, FontFamily, FontWeight, Widget, WidgetExt, Color, Insets, UnitPoint};
use druid::widget::{Controller, Flex, Label, List, Scroll, CrossAxisAlignment, LineBreaking, Either};

use crate::models::{Todo, TodoList, TodoProvider};

pub mod commands;

struct Sidenotes;

impl<W: Widget<TodoList>> Controller<TodoList, W> for Sidenotes {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut TodoList, env: &Env) {
        if let Event::Command(cmd) = event {
            if let Some(providers) = cmd.get(commands::PROVIDERS_CONFIGURED) {
                data.providers = providers.clone();
            }else if let Some((provider, todos)) = cmd.get(commands::TODOS_FETCHED) {
                data.providers[*provider].items = todos.clone();
            }
        }else {
            child.event(ctx, event, data, env)
        }
    }
}

pub fn ui_builder() -> impl Widget<TodoList> {
    let list = List::new(provider_builder)
        .lens(TodoList::providers);
    let list = Scroll::new(list).vertical();

    list
        .controller(Sidenotes)
}

fn provider_builder() -> impl Widget<TodoProvider> {
    let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(18.0)
        .with_weight(FontWeight::BOLD);
    let header = Label::new(|item: &TodoProvider, _env: &_| item.name.clone()).with_font(font)
        .align_horizontal(UnitPoint::CENTER);
    let todos = List::new(todo_builder)
        .lens(TodoProvider::items);

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
        .background(Color::from_hex_str("a3be8c").unwrap())
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
        without_state
    );

    state
        .padding(4.0)
        .background(Color::rgba8(0, 0, 0, 32))
        .rounded(2.0)
        .padding(Insets::uniform_xy(0., 2.))
        .expand_width()
}

fn title_builder() -> impl Widget<Todo> {
    Label::new(|item: &Todo, _env: &_| item.title.clone())
        .with_line_break_mode(LineBreaking::WordWrap)
}
