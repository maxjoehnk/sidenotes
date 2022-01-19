use crate::config::UiConfig;
use crate::models::{Todo, TodoProvider};
use druid::im::Vector;
use druid::Selector;

pub const FATAL_ERROR: Selector<String> = Selector::new("event-sidenotes.fatal-error");
pub const PROVIDER_ERROR: Selector<String> = Selector::new("event-sidenotes.provider-error");
pub const PROVIDERS_CONFIGURED: Selector<Vector<TodoProvider>> =
    Selector::new("event-sidenotes.providers-configured");
pub const TODOS_FETCHED: Selector<(usize, Vector<Todo>)> =
    Selector::new("event-sidenotes.todos-fetched");
pub const UI_CONFIG_LOADED: Selector<UiConfig> = Selector::new("event-sidenotes.ui-config-loaded");
pub const OPEN_TODO: Selector<Todo> = Selector::new("event-sidenotes.open-todo");
pub const CLOSE_TODO: Selector<()> = Selector::new("event-sidenotes.close-todo");
pub const OPEN_LINK: Selector<String> = Selector::new("sidenotes.open-link");
pub const TOGGLE_PROVIDER: Selector<TodoProvider> = Selector::new("sidenotes.TOGGLE_PROVIDER");
