use crate::models::Todo;
use crate::providers::{Provider, ProviderImpl, ProviderSettings};
use crate::ui::commands;
use druid::{ExtEventSink, Target};
use futures::FutureExt;
use im::Vector;
use std::thread;

pub struct FetchTodosJob {
    event_sink: ExtEventSink,
    providers: Vec<(ProviderSettings, ProviderImpl)>,
}

impl FetchTodosJob {
    pub fn new(event_sink: ExtEventSink, providers: Vec<(ProviderSettings, ProviderImpl)>) -> Self {
        Self {
            event_sink,
            providers,
        }
    }

    pub fn run(self) {
        thread::spawn(move || self.fetch_todos());
    }

    fn fetch_todos(&self) -> anyhow::Result<()> {
        let tasks = self
            .providers
            .iter()
            .enumerate()
            .map(|(index, (settings, provider))| {
                self.sync_provider(index, provider, settings).boxed_local()
            })
            .collect::<Vec<_>>();

        smol::block_on(futures::future::try_join_all(tasks))?;

        Ok(())
    }

    async fn sync_provider(
        &self,
        index: usize,
        provider: &impl Provider,
        settings: &ProviderSettings,
    ) -> anyhow::Result<()> {
        match provider.fetch_todos().await {
            Ok(todos) => {
                let todos = filter_todos(todos, settings);
                self.event_sink.submit_command(
                    commands::TODOS_FETCHED,
                    (index, todos),
                    Target::Auto,
                )?;
            }
            Err(err) => {
                tracing::error!("{:?}", err);
                self.event_sink.submit_command(
                    commands::PROVIDER_ERROR,
                    format!("{err}"),
                    Target::Auto,
                )?;
            }
        }
        Ok(())
    }
}

fn filter_todos(todos: Vector<Todo>, settings: &ProviderSettings) -> Vector<Todo> {
    todos
        .into_iter()
        .filter(|todo| {
            if let Some(state) = todo.state.as_ref() {
                !settings.exclude_status.contains(state)
            } else {
                true
            }
        })
        .collect()
}
