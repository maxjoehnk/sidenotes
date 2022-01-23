use crate::models::Appointment;
use chrono::Local;
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::{Deserialize, Serialize};

pub(crate) type TZ = Local;

#[cfg(feature = "c_ews")]
mod ews;

pub trait Calendar {
    fn next_appointment(&self) -> BoxFuture<anyhow::Result<Option<Appointment>>>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum CalendarConfig {
    #[cfg(feature = "c_ews")]
    Ews(ews::EwsConfig),
}

impl CalendarConfig {
    pub fn build(self) -> CalendarProvider {
        match self {
            #[cfg(feature = "c_ews")]
            Self::Ews(config) => CalendarProvider::Ews(config.into()),
        }
    }
}

pub enum CalendarProvider {
    #[cfg(feature = "c_ews")]
    Ews(ews::EwsClient),
}

impl Calendar for CalendarProvider {
    fn next_appointment(&self) -> BoxFuture<anyhow::Result<Option<Appointment>>> {
        match self {
            #[cfg(feature = "c_ews")]
            Self::Ews(ews) => ews.next_appointment(),
            #[allow(unreachable_patterns)]
            _ => futures::future::ok(None).boxed(),
        }
    }
}
