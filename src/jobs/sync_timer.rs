use crate::ui::commands;
use druid::{ExtEventSink, Target};
use std::thread;
use std::time::Duration;

pub struct SyncTimerJob(ExtEventSink, u64);

impl SyncTimerJob {
    pub fn new(event_sink: ExtEventSink, timeout: u64) -> Self {
        Self(event_sink, timeout)
    }

    pub fn run(self) {
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(self.1));
            if let Err(err) = self.run_timer() {
                tracing::error!("{err:?}");
            }
        });
    }

    fn run_timer(&self) -> anyhow::Result<()> {
        self.0
            .submit_command(commands::FETCH_TODOS, (), Target::Auto)?;
        self.0
            .submit_command(commands::FETCH_APPOINTMENTS, (), Target::Auto)?;
        self.0
            .submit_command(commands::TRIGGER_SYNC, (), Target::Auto)?;

        Ok(())
    }
}
