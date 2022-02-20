use crate::calendar::{Calendar, CalendarConfigEntry, CalendarProvider};
use crate::ui::commands;
use druid::{ExtEventSink, Target};
use std::thread;

pub struct FetchAppointmentsJob {
    event_sink: ExtEventSink,
    provider: Option<CalendarProvider>,
}

impl FetchAppointmentsJob {
    pub fn new<'a>(
        event_sink: ExtEventSink,
        mut config: impl Iterator<Item = &'a CalendarConfigEntry>,
    ) -> Self {
        let calendar_provider: Option<CalendarProvider> =
            config.next().map(|config| config.config.clone().build());

        Self {
            event_sink,
            provider: calendar_provider,
        }
    }

    pub fn run(self) {
        thread::spawn(move || {
            if let Some(ref provider) = self.provider {
                smol::block_on(self.sync_calendar(provider));
            }
        });
    }

    async fn sync_calendar(&self, provider: &impl Calendar) -> anyhow::Result<()> {
        match provider.next_appointment().await {
            Ok(appointment) => self.event_sink.submit_command(
                commands::NEXT_APPOINTMENT_FETCHED,
                appointment,
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
