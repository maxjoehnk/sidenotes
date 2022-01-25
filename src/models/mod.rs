use crate::calendar::TZ;
use chrono::DateTime;
use druid::im::Vector;
use druid::{lens, Data, Lens};

use crate::config::UiConfig;
use crate::providers::ProviderSettings;
use crate::rich_text::RawRichText;

#[derive(Default, Debug, Clone, Data, Lens)]
pub struct AppState {
    #[lens(ignore)]
    pub providers: Vector<TodoProvider>,
    pub next_appointment: Option<Appointment>,
    pub navigation: Navigation,
    pub ui_config: UiConfig,
}

impl AppState {
    pub fn providers() -> impl Lens<Self, Vector<TodoProvider>> {
        lens::Map::new::<Self, Vector<TodoProvider>>(
            |data| {
                data.providers
                    .iter()
                    .filter(|provider| {
                        !provider.items.is_empty() || !data.ui_config.hide_empty_providers
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
}

impl Default for Navigation {
    fn default() -> Self {
        Self::List
    }
}

impl Navigation {
    pub fn selected() -> impl Lens<Self, Todo> {
        lens::Map::new::<Self, Todo>(
            |data| {
                if let Navigation::Selected(ref todo) = data {
                    todo.clone()
                } else {
                    unreachable!()
                }
            },
            |_, _| {},
        )
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
    pub title: String,
    pub state: Option<String>,
    pub tags: Vector<String>,
    pub author: Option<String>,
    pub body: Option<RawRichText>,
    pub link: Option<String>,
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
