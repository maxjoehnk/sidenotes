use crate::calendar::{Appointment, Calendar};
pub(crate) use ews_calendar::EwsClient;
use ews_calendar::ExchangeVersion::Exchange2016;
use futures::future::BoxFuture;
use futures::FutureExt;
use im::Vector;
use serde::{Deserialize, Serialize};

impl Calendar for EwsClient {
    fn todays_appointments(&self) -> BoxFuture<anyhow::Result<Vector<Appointment>>> {
        async move {
            let items = self.find_items().await?;
            let appointments = items
                .into_iter()
                .map(|item| Appointment {
                    title: item.subject.clone(),
                    description: Default::default(),
                    start_time: item.start,
                    end_time: item.end,
                    meeting_link: None,
                })
                .collect();

            Ok(appointments)
        }
        .boxed()
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, druid::Data, druid::Lens)]
pub struct EwsConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl From<EwsConfig> for EwsClient {
    fn from(config: EwsConfig) -> Self {
        Self::new(config.url, Exchange2016, config.username, config.password)
    }
}
