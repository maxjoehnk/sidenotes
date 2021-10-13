use druid::{Data, Lens};
use druid::im::Vector;

#[derive(Default, Debug, Clone, Data, Lens)]
pub struct TodoList {
    pub providers: Vector<TodoProvider>
}

#[derive(Debug, Clone, Data, Lens)]
pub struct TodoProvider {
    pub name: String,
    pub items: Vector<Todo>
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub state: Option<String>
}
