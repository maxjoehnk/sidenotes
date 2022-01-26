use crate::config::Config;
use crate::models::*;
use crate::providers::{ProviderId, ProviderImpl, ProviderSettings};
use druid::im::Vector;
use druid::Selector;
use std::collections::HashMap;

pub const FATAL_ERROR: Selector<String> = Selector::new("event-sidenotes.fatal-error");
pub const PROVIDER_ERROR: Selector<String> = Selector::new("event-sidenotes.provider-error");
pub const PROVIDERS_CONFIGURED: Selector<HashMap<ProviderId, (ProviderSettings, ProviderImpl)>> =
    Selector::new("event-sidenotes.providers-configured");
pub const FETCH_TODOS: Selector<()> = Selector::new("sidenotes.fetch-todos");
pub const TODOS_FETCHED: Selector<(usize, Vector<Todo>)> =
    Selector::new("event-sidenotes.todos-fetched");
pub const COMMENTS_FETCHED: Selector<(TodoId, Vector<TodoComment>)> =
    Selector::new("event-sidenotes.comments-fetched");
pub const CONFIG_LOADED: Selector<Config> = Selector::new("event-sidenotes.config-loaded");
pub const OPEN_TODO: Selector<Todo> = Selector::new("event-sidenotes.open-todo");
pub const CLOSE_TODO: Selector<()> = Selector::new("event-sidenotes.close-todo");
pub const OPEN_LINK: Selector<String> = Selector::new("sidenotes.open-link");
pub const TOGGLE_PROVIDER: Selector<TodoProvider> = Selector::new("sidenotes.TOGGLE_PROVIDER");
pub const PROVIDER_ACTION: Selector<TodoAction> = Selector::new("sidenotes.provider-action");

pub const FETCH_APPOINTMENTS: Selector<()> = Selector::new("sidenotes.fetch-appointments");
pub const NEXT_APPOINTMENT_FETCHED: Selector<Option<Appointment>> =
    Selector::new("event-sidenotes.next-appointment-fetched");
