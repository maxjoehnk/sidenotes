use druid::{Env, Event, EventCtx, FontDescriptor, FontFamily, FontWeight, Widget, WidgetExt, UnitPoint};
use druid::widget::{Checkbox, Controller, Flex, Label, List, Scroll};

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
        .lens(TodoList::providers)
        .expand_width();
    let list = Scroll::new(list).vertical();

    list
        .controller(Sidenotes)
}

fn provider_builder() -> impl Widget<TodoProvider> {
    let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(18.0)
        .with_weight(FontWeight::BOLD);
    let header = Label::new(|item: &TodoProvider, _env: &_| item.name.clone()).with_font(font);
    let todos = List::new(todo_builder)
        .lens(TodoProvider::items);

    Flex::column()
        .with_child(header)
        .with_child(todos)
        .align_vertical(UnitPoint::CENTER)
}

fn todo_builder() -> impl Widget<Todo> {
    let title = Label::new(|item: &Todo, _env: &_| item.title.clone());
    let checkbox = Checkbox::new(String::new())
        .disabled_if(|_, _| true)
        .lens(Todo::completed);
    let header = Flex::row()
        .with_child(checkbox)
        .with_child(title);

    let state = Label::new(|item: &Todo, _env: &_| item.state.clone().unwrap_or_default());

    Flex::column()
        .with_child(header)
        .with_child(state)
        .padding(8.)
        .align_left()
        .expand_width()
}
