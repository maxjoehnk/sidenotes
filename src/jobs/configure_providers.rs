use crate::config::Config;
use crate::providers::{ProviderImpl, ProviderSettings};
use crate::ui::commands;
use druid::{ExtEventSink, Target};
use futures::future::BoxFuture;
use futures::FutureExt;
use std::thread;

pub struct ConfigureProvidersJob {
    event_sink: ExtEventSink,
    config: Config,
}

impl ConfigureProvidersJob {
    pub fn new(config: Config, event_sink: ExtEventSink) -> Self {
        Self { config, event_sink }
    }

    pub fn run(self) {
        thread::spawn(move || {
            self.configure_providers();
        });
    }

    fn configure_providers(self) -> anyhow::Result<()> {
        let providers: Vec<Option<(ProviderSettings, ProviderImpl)>> =
            smol::block_on(futures::future::try_join_all(
                self.config.providers.into_iter().map::<BoxFuture<
                    anyhow::Result<Option<(ProviderSettings, ProviderImpl)>>,
                >, _>(|provider_config| {
                    async {
                        match provider_config.provider.create().await {
                            Ok(provider) => Ok(Some((provider_config.settings, provider))),
                            Err(err) => {
                                tracing::error!("Error setting up provider: {:?}", err);
                                Ok(None)
                            }
                        }
                    }
                    .boxed()
                }),
            ))?;

        let providers = providers.into_iter().flatten().collect::<Vec<_>>();

        self.event_sink
            .submit_command(commands::PROVIDERS_CONFIGURED, providers, Target::Auto)?;

        Ok(())
    }
}
