use crate::models::AppState;
use crate::ui::commands;
use druid::{ExtEventSink, Target};
use std::thread;
use std::time::Duration;

pub struct SyncTimerJob(ExtEventSink);

impl SyncTimerJob {
    pub fn new(event_sink: ExtEventSink) -> Self {
        Self(event_sink)
    }

    pub fn run(self) {
        self.queue_job()
    }

    fn queue_job(self) {
        let event_sink = self.0.clone();
        event_sink.add_idle_callback(move |state: &mut AppState| {
            let timeout = state.config.sync_timeout;
            thread::spawn(move || {
                self.0
                    .submit_command(commands::FETCH_TODOS, (), Target::Auto);
                self.0
                    .submit_command(commands::FETCH_APPOINTMENTS, (), Target::Auto);
                thread::sleep(Duration::from_secs(timeout));
                self.queue_job();
            });
        });
    }
}
