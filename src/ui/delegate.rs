use crate::calendar::CalendarConfigEntry;
use crate::config::{get_config_save_path, save, Config};
use crate::jobs::{
    ConfigureProvidersJob, FetchAppointmentsJob, FetchCommentsJob, FetchTodosJob, ProviderActionJob,
};
use druid::{AppDelegate, Command, DelegateCtx, Env, ExtEventSink, Handled, Target};
use im::Vector;
use std::collections::HashMap;
use tracing::log;

use crate::models::{AppState, Navigation, Todo, TodoAction, TodoProvider};
use crate::providers::{Provider, ProviderId, ProviderImpl, ProviderSettings};
use crate::ui::commands;
use crate::SyncTimerJob;

#[derive(Default)]
pub(crate) struct SidenotesDelegate {
    providers: HashMap<ProviderId, (ProviderSettings, ProviderImpl)>,
}

impl AppDelegate<AppState> for SidenotesDelegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _: Target,
        cmd: &Command,
        data: &mut AppState,
        _: &Env,
    ) -> Handled {
        tracing::debug!("Handling command {:?}", cmd);
        if let Some((config, path)) = cmd.get(commands::CONFIG_LOADED) {
            data.config = config.clone();
            data.config_path = Some(path.clone());
            SyncTimerJob::new(ctx.get_external_handle(), config.sync_timeout).run();
            Self::reconfigure_providers(ctx, config);
        } else if let Some(providers) = cmd.get(commands::PROVIDERS_CONFIGURED) {
            data.providers = Self::map_providers(providers.values());
            self.providers = providers.clone();
            ctx.submit_command(Command::new(commands::FETCH_TODOS, (), Target::Auto));
            ctx.submit_command(Command::new(commands::FETCH_APPOINTMENTS, (), Target::Auto));
        } else if cmd.is(commands::FETCH_TODOS) {
            FetchTodosJob::new(
                ctx.get_external_handle(),
                self.providers.values().cloned().collect(),
            )
            .run();
        } else if cmd.is(commands::FETCH_APPOINTMENTS) {
            FetchAppointmentsJob::new(
                ctx.get_external_handle(),
                data.config.calendar_config.iter(),
            )
            .run();
        } else if let Some(action) = cmd.get(commands::PROVIDER_ACTION) {
            if let Navigation::Selected(ref todo) = data.navigation {
                self.run_provider_action(todo, action, ctx.get_external_handle());
            }
        } else if let Some((provider, todos)) = cmd.get(commands::TODOS_FETCHED) {
            data.providers[*provider].items = todos.clone();
        } else if let Some((todo_id, comments)) = cmd.get(commands::COMMENTS_FETCHED) {
            if let Navigation::Selected(todo) = &mut data.navigation {
                if &todo.id == todo_id {
                    todo.comments = comments.clone();
                }
            }
        } else if let Some(navigation) = cmd.get(commands::NAVIGATE) {
            if let Navigation::Selected(todo) = navigation {
                self.fetch_comments(todo, ctx.get_external_handle());
            }
            data.navigation = navigation.clone();
        } else if cmd.get(commands::NAVIGATE_BACK).is_some() {
            let next = match data.navigation {
                Navigation::EditProvider(_) | Navigation::NewProvider => {
                    Navigation::ProviderSettings
                }
                Navigation::EditCalendar(_) | Navigation::NewCalendar => {
                    Navigation::CalendarSettings
                }
                Navigation::ProviderSettings
                | Navigation::CalendarSettings
                | Navigation::GlobalSettings(_) => Navigation::Settings,
                _ => Navigation::List,
            };
            data.navigation = next;
        } else if let Some(link) = cmd.get(commands::OPEN_LINK) {
            if let Err(err) = open::that(link) {
                log::error!("Error opening {link}: {err}");
            }
        } else if let Some(appointments) = cmd.get(commands::APPOINTMENTS_FETCHED) {
            data.appointments = appointments.clone();
        } else if let Some(provider) = cmd.get(commands::TOGGLE_PROVIDER) {
            if let Some(index) = data.providers.iter().position(|p| p.name == provider.name) {
                data.providers[index].collapsed = !data.providers[index].collapsed;
            }
        } else if cmd.get(commands::SAVE_PROVIDER).is_some() {
            let mut navigation = Navigation::ProviderSettings;
            std::mem::swap(&mut navigation, &mut data.navigation);
            if let Navigation::EditProvider(config) = navigation {
                if let Some(provider) = data.config.providers.iter_mut().find(|p| p.id == config.id)
                {
                    provider.provider = config.provider;
                    provider.settings = config.settings;
                } else {
                    data.config.providers.push_back(config);
                }
                Self::reconfigure_providers(ctx, &data.config);
            }
            Self::save_config(data);
        } else if cmd.get(commands::DELETE_PROVIDER).is_some() {
            let mut navigation = Navigation::ProviderSettings;
            std::mem::swap(&mut navigation, &mut data.navigation);
            if let Navigation::EditProvider(config) = navigation {
                if let Some(index) = data.config.providers.iter().position(|p| p.id == config.id) {
                    data.config.providers.remove(index);
                    Self::reconfigure_providers(ctx, &data.config);
                }
            }
            Self::save_config(data);
        } else if cmd.get(commands::SAVE_CALENDAR).is_some() {
            let mut navigation = Navigation::CalendarSettings;
            std::mem::swap(&mut navigation, &mut data.navigation);
            if let Navigation::EditCalendar((id, config)) = navigation {
                if let Some(calendar) = data.config.calendar_config.iter_mut().find(|p| p.id == id)
                {
                    calendar.config = config;
                } else {
                    data.config
                        .calendar_config
                        .push_back(CalendarConfigEntry { id, config });
                }
            }
            Self::save_config(data);
        } else if cmd.get(commands::DELETE_CALENDAR).is_some() {
            let mut navigation = Navigation::CalendarSettings;
            std::mem::swap(&mut navigation, &mut data.navigation);
            if let Navigation::EditCalendar((id, _)) = navigation {
                if let Some(index) = data.config.calendar_config.iter().position(|p| p.id == id) {
                    data.config.calendar_config.remove(index);
                }
            }
            Self::save_config(data);
        } else if cmd.is(commands::SAVE_GLOBAL_CONFIG) {
            let mut navigation = Navigation::Settings;
            std::mem::swap(&mut navigation, &mut data.navigation);
            if let Navigation::GlobalSettings(config) = navigation {
                data.config.sync_timeout = config.sync_timeout;
                data.config.ui = config.ui;
            }
            Self::save_config(data);
        } else if cmd.is(commands::TRIGGER_SYNC) {
            SyncTimerJob::new(ctx.get_external_handle(), data.config.sync_timeout).run();
        } else {
            return Handled::No;
        }
        Handled::Yes
    }
}

impl SidenotesDelegate {
    fn run_provider_action(&self, todo: &Todo, action: &TodoAction, event_sink: ExtEventSink) {
        if let Some((_, provider)) = self.providers.get(&todo.provider) {
            ProviderActionJob::new(event_sink, provider.clone(), todo.id.clone(), *action).run();
        }
    }

    fn fetch_comments(&self, todo: &Todo, event_sink: ExtEventSink) {
        if let Some((_, provider)) = self.providers.get(&todo.provider) {
            FetchCommentsJob::new(event_sink, provider.clone(), todo.id.clone()).run();
        }
    }

    fn map_providers<'a>(
        providers: impl Iterator<Item = &'a (ProviderSettings, ProviderImpl)>,
    ) -> Vector<TodoProvider> {
        providers
            .map(|(settings, provider)| TodoProvider {
                name: settings
                    .name
                    .clone()
                    .unwrap_or_else(|| provider.name().to_string()),
                items: Default::default(),
                settings: settings.clone(),
                collapsed: false,
            })
            .collect()
    }

    fn reconfigure_providers(ctx: &mut DelegateCtx, config: &Config) {
        ConfigureProvidersJob::new(config.clone(), ctx.get_external_handle()).run();
    }

    fn save_config(data: &mut AppState) {
        if data.config_path.is_none() {
            data.config_path = Some(get_config_save_path());
        }
        if let Some(path) = &data.config_path {
            if let Err(err) = save(path, &data.config) {
                tracing::error!("Saving config failed {:?}", err);
            }
        }
    }
}
