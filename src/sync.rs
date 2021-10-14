use std::thread;
use std::time::Duration;

use druid::{ExtEventSink, Target};
use druid::im::Vector;

use crate::models::TodoProvider;
use crate::ui::commands;
use crate::providers::Provider;

pub struct SyncThread {
    event_sink: ExtEventSink,
}

impl SyncThread {
    pub fn new(event_sink: ExtEventSink) -> Self {
        Self {
            event_sink
        }
    }

    pub fn start(self) {
        thread::spawn(move || {
            if let Err(err) = self.run() {
                tracing::error!("{:?}", err);
                self.event_sink.submit_command(commands::FATAL_ERROR, format!("{:?}", err), Target::Auto).unwrap();
            }
        });
    }

    fn run(&self) -> anyhow::Result<()> {
        let config = crate::config::load()?;
        self.event_sink.submit_command(commands::UI_CONFIG_LOADED, config.ui, Target::Auto)?;
        let providers = config.providers.into_iter()
            .map(|provider_config| provider_config.create())
            .collect::<anyhow::Result<Vec<_>>>()?;

        let todo_providers: Vector<TodoProvider> = providers.iter().map(|provider| TodoProvider {
            name: provider.name(),
            items: Default::default(),
        }).collect();

        self.event_sink.submit_command(commands::PROVIDERS_CONFIGURED, todo_providers, Target::Auto)?;

        loop {
            let tasks = providers
                .iter()
                .enumerate()
                .map(|(index, provider)| self.sync_provider(index, provider));

            smol::block_on(futures::future::try_join_all(tasks))?;
            thread::sleep(Duration::from_secs(config.sync_timeout));
        }
    }

    async fn sync_provider(&self, index: usize, provider: &Box<dyn Provider>) -> anyhow::Result<()> {
        match provider.fetch_todos().await {
            Ok(todos) => {
                self.event_sink.submit_command(commands::TODOS_FETCHED, (index, todos), Target::Auto)?;
            },
            Err(err) => {
                tracing::error!("{:?}", err);
                self.event_sink.submit_command(commands::PROVIDER_ERROR, format!("{}", err), Target::Auto)?;
            }
        }
        Ok(())
    }
}
