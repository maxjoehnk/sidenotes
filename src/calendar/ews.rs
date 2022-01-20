use futures::future::BoxFuture;
use futures::FutureExt;
use serde::{Deserialize, Serialize};
pub(crate) use ews_calendar::EwsClient;
use ews_calendar::ExchangeVersion::Exchange2016;
use crate::calendar::{Appointment, Calendar};

impl Calendar for EwsClient {
    fn next_appointment(&self) -> BoxFuture<anyhow::Result<Option<Appointment>>> {
        async move {
            let items = self.find_items().await?;
            let appointment = items.first().map(|item| Appointment {
                title: item.subject.clone(),
                description: Default::default(),
                start_time: item.start,
                end_time: item.end,
                meeting_link: None,
            });

            Ok(appointment)
        }.boxed()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
