use chrono::Local;
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use crate::models::Appointment;

pub(crate) type TZ = Local;

#[cfg(feature = "ews-calendar")]
mod ews;

pub trait Calendar {
    fn next_appointment(&self) -> BoxFuture<anyhow::Result<Option<Appointment>>>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum CalendarConfig {
    #[cfg(feature = "ews-calendar")]
    Ews(ews::EwsConfig)
}

impl CalendarConfig {
    pub fn build(self) -> CalendarProvider {
        match self {
            #[cfg(feature = "ews-calendar")]
            Self::Ews(config) => CalendarProvider::Ews(config.into()),
        }
    }
}

pub enum CalendarProvider {
    #[cfg(feature = "ews-calendar")]
    Ews(ews::EwsClient)
}

impl Calendar for CalendarProvider {
    fn next_appointment(&self) -> BoxFuture<anyhow::Result<Option<Appointment>>> {
        match self {
            #[cfg(feature = "ews-calendar")]
            Self::Ews(ews) => ews.next_appointment(),
        }
    }
}
