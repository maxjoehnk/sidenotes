use crate::config::Config;
use crate::providers::{ProviderId, ProviderImpl, ProviderSettings};
use crate::ui::commands;
use druid::{ExtEventSink, Target};
use futures::future::BoxFuture;
use futures::FutureExt;
use std::collections::HashMap;
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
            if let Err(err) = self.configure_providers() {
                tracing::error!("{err:?}");
            }
        });
    }

    fn configure_providers(self) -> anyhow::Result<()> {
        let providers: Vec<Option<(ProviderId, (ProviderSettings, ProviderImpl))>> =
            smol::block_on(futures::future::join_all(
                self.config.providers.into_iter().map::<BoxFuture<
                    Option<(ProviderId, (ProviderSettings, ProviderImpl))>,
                >, _>(|provider_config| {
                    async {
                        match provider_config.provider.create(provider_config.id).await {
                            Ok(provider) => {
                                Some((provider_config.id, (provider_config.settings, provider)))
                            }
                            Err(err) => {
                                tracing::error!("Error setting up provider: {:?}", err);
                                None
                            }
                        }
                    }
                    .boxed()
                }),
            ));

        let providers = providers.into_iter().flatten().collect::<HashMap<_, _>>();

        self.event_sink
            .submit_command(commands::PROVIDERS_CONFIGURED, providers, Target::Auto)?;

        Ok(())
    }
}
