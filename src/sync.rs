use std::thread;
use std::time::Duration;

use druid::im::Vector;
use druid::{ExtEventSink, Target};
use futures::future::BoxFuture;
use futures::FutureExt;

use crate::models::{Todo, TodoProvider};
use crate::providers::{Provider, ProviderSettings};
use crate::ui::commands;

pub struct SyncThread {
    event_sink: ExtEventSink,
}

impl SyncThread {
    pub fn new(event_sink: ExtEventSink) -> Self {
        Self { event_sink }
    }

    pub fn start(self) {
        thread::spawn(move || {
            if let Err(err) = self.run() {
                tracing::error!("{:?}", err);
                self.event_sink
                    .submit_command(commands::FATAL_ERROR, format!("{:?}", err), Target::Auto)
                    .unwrap();
            }
        });
    }

    fn run(&self) -> anyhow::Result<()> {
        let config = crate::config::load()?;
        self.event_sink
            .submit_command(commands::UI_CONFIG_LOADED, config.ui, Target::Auto)?;
        let providers: Vec<(ProviderSettings, Box<dyn Provider>)> = smol::block_on(futures::future::try_join_all(
            config
                .providers
                .into_iter()
                .map::<BoxFuture<anyhow::Result<(ProviderSettings, Box<dyn Provider>)>>, _>(|provider_config| {
                    async {
                        let provider = provider_config.provider.create().await?;

                        Ok((provider_config.settings, provider))
                    }.boxed()
                }),
        ))?;

        let todo_providers: Vector<TodoProvider> = providers
            .iter()
            .map(|(settings, provider)| TodoProvider {
                name: provider.name(),
                items: Default::default(),
                settings: settings.clone(),
            })
            .collect();

        self.event_sink.submit_command(
            commands::PROVIDERS_CONFIGURED,
            todo_providers,
            Target::Auto,
        )?;

        loop {
            let tasks = providers
                .iter()
                .enumerate()
                .map(|(index, (settings, provider))| self.sync_provider(index, provider.as_ref(), settings));

            smol::block_on(futures::future::try_join_all(tasks))?;
            thread::sleep(Duration::from_secs(config.sync_timeout));
        }
    }

    async fn sync_provider(&self, index: usize, provider: &dyn Provider, settings: &ProviderSettings) -> anyhow::Result<()> {
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
                    format!("{}", err),
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
            }else {
                true
            }
        })
        .collect()
}