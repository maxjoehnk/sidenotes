use crate::ui::commands;
use druid::{ExtEventSink, Target};
use std::thread;

pub struct ConfigLoadJob(ExtEventSink);

impl ConfigLoadJob {
    pub fn new(event_sink: ExtEventSink) -> Self {
        Self(event_sink)
    }

    pub fn run(self) {
        thread::spawn(move || {
            if let Err(err) = self.load_config() {
                tracing::error!("{:?}", err);
                self.0
                    .submit_command(commands::FATAL_ERROR, format!("{:?}", err), Target::Auto)
                    .unwrap();
            }
        });
    }

    fn load_config(&self) -> anyhow::Result<()> {
        let config = crate::config::load()?;
        self.0
            .submit_command(commands::CONFIG_LOADED, config, Target::Auto)?;

        Ok(())
    }
}
