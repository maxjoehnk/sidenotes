use crate::models::TodoId;
use crate::providers::{Provider, ProviderImpl};
use crate::ui::commands;
use druid::{ExtEventSink, Target};
use std::thread;

pub struct FetchCommentsJob {
    event_sink: ExtEventSink,
    todo_id: TodoId,
    provider: ProviderImpl,
}

impl FetchCommentsJob {
    pub fn new(event_sink: ExtEventSink, provider: ProviderImpl, todo_id: TodoId) -> Self {
        Self {
            event_sink,
            provider,
            todo_id,
        }
    }

    pub fn run(self) {
        thread::spawn(move || {
            smol::block_on(self.fetch_comments());
        });
    }

    async fn fetch_comments(self) -> anyhow::Result<()> {
        let comments = self.provider.fetch_comments(self.todo_id.clone()).await?;
        self.event_sink.submit_command(
            commands::COMMENTS_FETCHED,
            (self.todo_id, comments),
            Target::Auto,
        )?;

        Ok(())
    }
}
