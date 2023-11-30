use crate::calendar::{CalendarConfig, CalendarId, TZ};
use chrono::{DateTime, Local, TimeZone};
use druid::im::Vector;
use druid::{lens, Data, Lens};
use std::ops::Deref;
use std::path::PathBuf;

use crate::config::Config;
use crate::providers::{ProviderConfigEntry, ProviderId, ProviderSettings};
use crate::rich_text::RawRichText;

#[derive(Default, Debug, Clone, Data, Lens)]
pub struct AppState {
    #[lens(ignore)]
    pub providers: Vector<TodoProvider>,
    pub appointments: Vector<Appointment>,
    pub navigation: Navigation,
    pub config: Config,
    #[data(ignore)]
    pub config_path: Option<PathBuf>,
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

impl AppState {
    #[allow(non_upper_case_globals)]
    pub const next_appointment: NextAppointmentLens = NextAppointmentLens;
}

#[derive(Clone, Copy)]
pub struct NextAppointmentLens;

impl Lens<AppState, Option<Appointment>> for NextAppointmentLens {
    fn with<V, F: FnOnce(&Option<Appointment>) -> V>(&self, data: &AppState, f: F) -> V {
        f(&data.appointments.front().cloned())
    }

    fn with_mut<V, F: FnOnce(&mut Option<Appointment>) -> V>(
        &self,
        data: &mut AppState,
        f: F,
    ) -> V {
        f(&mut data.appointments.front().cloned())
    }
}

#[derive(Debug, Clone, Data)]
#[allow(clippy::large_enum_variant)]
pub enum Navigation {
    List,
    Selected(Todo),
    Settings,
    GlobalSettings(Config),
    ProviderSettings,
    CalendarSettings,
    NewProvider,
    EditProvider(ProviderConfigEntry),
    NewCalendar,
    EditCalendar((CalendarId, CalendarConfig)),
    Appointments,
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
    pub due_date: Option<LocalDateTime>,
}

impl Todo {
    pub fn labels() -> impl Lens<Self, Vector<String>> {
        lens::Map::new(
            |todo: &Todo| {
                let mut labels = Vector::new();
                if let Some(state) = &todo.state {
                    labels.push_back(state.clone());
                }
                for tag in &todo.tags {
                    labels.push_back(tag.clone());
                }

                labels
            },
            |_, _| {},
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct LocalDateTime(DateTime<TZ>);

impl Data for LocalDateTime {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<DateTime<TZ>> for LocalDateTime {
    fn from(time: DateTime<TZ>) -> Self {
        Self(time)
    }
}

impl Deref for LocalDateTime {
    type Target = DateTime<TZ>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LocalDateTime {
    pub fn from_timestamp(timestamp: u64) -> Self {
        Self(Local.timestamp_millis_opt(timestamp as i64).unwrap())
    }

    pub fn is_today(&self) -> bool {
        self.0.date_naive() == TZ::now().date_naive()
    }
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
