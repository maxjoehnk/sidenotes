use crate::calendar::{Calendar, CalendarConfigEntry, CalendarProvider};
use crate::ui::commands;
use druid::{ExtEventSink, Target};
use im::Vector;
use itertools::Itertools;
use std::thread;

pub struct FetchAppointmentsJob {
    event_sink: ExtEventSink,
    providers: Vec<CalendarProvider>,
}

impl FetchAppointmentsJob {
    pub fn new<'a>(
        event_sink: ExtEventSink,
        config: impl Iterator<Item = &'a CalendarConfigEntry>,
    ) -> Self {
        Self {
            event_sink,
            providers: config.map(|config| config.config.clone().build()).collect(),
        }
    }

    pub fn run(self) {
        thread::spawn(move || {
            if let Err(err) = smol::block_on(self.sync_calendars(&self.providers)) {
                tracing::error!("{err:?}");
            }
        });
    }

    async fn sync_calendars(&self, providers: &[CalendarProvider]) -> anyhow::Result<()> {
        let result = futures::future::try_join_all(
            providers
                .iter()
                .map(|provider| provider.todays_appointments()),
        )
        .await;
        match result {
            Ok(appointments) => self.event_sink.submit_command(
                commands::APPOINTMENTS_FETCHED,
                appointments
                    .into_iter()
                    .flatten()
                    .sorted_by_key(|a| a.start_time)
                    .collect::<Vector<_>>(),
                Target::Auto,
            )?,
            Err(err) => {
                tracing::error!("{:?}", err);
                self.event_sink.submit_command(
                    commands::PROVIDER_ERROR,
                    format!("{}", err),
                    Target::Auto,
                )?;
            }
        }
        Ok(())
    }
}
