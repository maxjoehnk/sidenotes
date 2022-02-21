use crate::models::Appointment;
use chrono::Local;
use derive_more::From;
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

pub(crate) type TZ = Local;

#[cfg(feature = "ews-calendar")]
pub mod ews;

pub trait Calendar {
    fn next_appointment(&self) -> BoxFuture<anyhow::Result<Option<Appointment>>>;
}

#[derive(Debug, Clone, Deserialize, Serialize, From, druid::Data)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum CalendarConfig {
    #[cfg(feature = "ews-calendar")]
    Ews(ews::EwsConfig),
}

impl CalendarConfig {
    pub fn build(self) -> CalendarProvider {
        match self {
            #[cfg(feature = "ews-calendar")]
            Self::Ews(config) => CalendarProvider::Ews(config.into()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, druid::Data, druid::Lens)]
pub struct CalendarConfigEntry {
    #[serde(default, skip)]
    pub id: CalendarId,
    #[serde(flatten)]
    pub config: CalendarConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CalendarId(uuid::Uuid);

impl Default for CalendarId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl druid::Data for CalendarId {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub enum CalendarProvider {
    #[cfg(feature = "ews-calendar")]
    Ews(ews::EwsClient),
}

impl Calendar for CalendarProvider {
    fn next_appointment(&self) -> BoxFuture<anyhow::Result<Option<Appointment>>> {
        match self {
            #[cfg(feature = "ews-calendar")]
            Self::Ews(ews) => ews.next_appointment(),
        }
    }
}
