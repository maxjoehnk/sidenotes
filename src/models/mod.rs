use druid::{Data, Lens};
use druid::im::Vector;

use crate::config::UiConfig;

#[derive(Default, Debug, Clone, Data, Lens)]
pub struct TodoList {
    #[lens(name = "all_providers")]
    pub providers: Vector<TodoProvider>,
    pub ui_config: UiConfig,
}

impl TodoList {
    pub fn providers() -> impl Lens<TodoList, Vector<TodoProvider>> {
        druid::lens::Map::new::<TodoList, Vector<TodoProvider>>(|data| {
            data.providers.iter().filter(|provider| {
                !provider.items.is_empty() || !data.ui_config.hide_empty_providers
            }).cloned().collect()
        }, |_, _| {})
    }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct TodoProvider {
    pub name: String,
    pub items: Vector<Todo>,
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub state: Option<String>,
}
