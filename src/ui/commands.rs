use druid::Selector;
use crate::models::{TodoProvider, Todo};
use druid::im::Vector;
use crate::config::UiConfig;

pub const FATAL_ERROR: Selector<String> = Selector::new("event-sidenotes.fatal-error");
pub const PROVIDER_ERROR: Selector<String> = Selector::new("event-sidenotes.provider-error");
pub const PROVIDERS_CONFIGURED: Selector<Vector<TodoProvider>> = Selector::new("event-sidenotes.providers-configured");
pub const TODOS_FETCHED: Selector<(usize, Vector<Todo>)> = Selector::new("event-sidenotes.todos-fetched");
pub const UI_CONFIG_LOADED: Selector<UiConfig> = Selector::new("event-sidenotes.ui-config-loaded");
