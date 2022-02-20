use crate::models::{TodoAction, TodoId};
use crate::providers::{Provider, ProviderImpl};
use crate::ui::commands;
use druid::{ExtEventSink, Target};
use std::thread;

pub struct ProviderActionJob {
    event_sink: ExtEventSink,
    provider: ProviderImpl,
    todo_id: TodoId,
    action: TodoAction,
}

impl ProviderActionJob {
    pub fn new(
        event_sink: ExtEventSink,
        provider: ProviderImpl,
        todo_id: TodoId,
        action: TodoAction,
    ) -> Self {
        Self {
            event_sink,
            provider,
            todo_id,
            action,
        }
    }

    pub fn run(self) {
        thread::spawn(move || {
            smol::block_on(self.run_action());
        });
    }

    async fn run_action(self) -> anyhow::Result<()> {
        self.provider.run_action(self.todo_id, self.action).await?;
        self.event_sink
            .submit_command(commands::NAVIGATE_BACK, (), Target::Auto)?;
        self.event_sink
            .submit_command(commands::FETCH_TODOS, (), Target::Auto)?;

        Ok(())
    }
}
