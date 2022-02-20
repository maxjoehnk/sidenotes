use crate::calendar::TZ;
use chrono::DateTime;
use druid::im::Vector;
use druid::{lens, Data, Lens};
use std::path::PathBuf;

use crate::config::Config;
use crate::providers::{ProviderConfig, ProviderId, ProviderSettings};
use crate::rich_text::RawRichText;

#[derive(Default, Debug, Clone, Data, Lens)]
pub struct AppState {
    #[lens(ignore)]
    pub providers: Vector<TodoProvider>,
    pub next_appointment: Option<Appointment>,
    pub navigation: Navigation,
    pub config: Config,
    #[data(ignore)]
    pub config_path: PathBuf,
}

impl AppState {
    pub fn providers() -> impl Lens<Self, Vector<TodoProvider>> {
        lens::Map::new::<Self, Vector<TodoProvider>>(
            |data| {
                data.providers
                    .iter()
                    .filter(|provider| {
                        !provider.items.is_empty() || !data.config.ui.hide_empty_providers
                    })
                    .cloned()
                    .collect()
            },
            |_, _| {},
        )
    }
}

#[derive(Debug, Clone, Data)]
pub enum Navigation {
    List,
    Selected(Todo),
    Settings,
    GlobalSettings(Config),
    ProviderSettings,
    CalendarSettings,
    NewProvider,
    EditProvider((ProviderId, ProviderConfig)),
}

impl Default for Navigation {
    fn default() -> Self {
        Self::List
    }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct TodoProvider {
    pub name: String,
    pub items: Vector<Todo>,
    #[lens(ignore)]
    pub settings: ProviderSettings,
    pub collapsed: bool,
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Todo {
    pub provider: ProviderId,
    pub id: TodoId,
    pub title: String,
    pub state: Option<String>,
    pub tags: Vector<String>,
    pub author: Option<String>,
    pub body: Option<RawRichText>,
    pub link: Option<String>,
    pub actions: Vector<TodoAction>,
    pub comments: Vector<TodoComment>,
}

#[derive(Debug, Clone, Copy, Data)]
pub struct TodoAction {
    pub id: &'static str,
    pub label: &'static str,
}

#[derive(Debug, Clone, Data, PartialEq, Eq)]
pub enum TodoId {
    String(String),
    Int(u64),
}

impl From<String> for TodoId {
    fn from(id: String) -> Self {
        Self::String(id)
    }
}

impl From<u64> for TodoId {
    fn from(id: u64) -> Self {
        Self::Int(id)
    }
}

impl From<u32> for TodoId {
    fn from(id: u32) -> Self {
        Self::Int(id as u64)
    }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct TodoComment {
    pub text: RawRichText,
    pub author: Option<String>,
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Appointment {
    pub title: String,
    pub description: String,
    #[data(ignore)]
    pub start_time: DateTime<TZ>,
    #[data(ignore)]
    pub end_time: DateTime<TZ>,
    pub meeting_link: Option<String>,
}
