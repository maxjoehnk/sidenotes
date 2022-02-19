use crate::jobs::{ConfigureProvidersJob, FetchAppointmentsJob, FetchTodosJob};
use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};
use im::Vector;

use crate::models::{AppState, Navigation, TodoProvider};
use crate::providers::{Provider, ProviderImpl, ProviderSettings};
use crate::ui::commands;

#[derive(Default)]
pub(crate) struct SidenotesDelegate {
    providers: Vec<(ProviderSettings, ProviderImpl)>,
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
        if let Some(config) = cmd.get(commands::CONFIG_LOADED) {
            data.config = config.clone();
            ConfigureProvidersJob::new(config.clone(), ctx.get_external_handle()).run();
        } else if let Some(providers) = cmd.get(commands::PROVIDERS_CONFIGURED) {
            data.providers = Self::map_providers(providers);
            self.providers = providers.clone();
            ctx.submit_command(Command::new(commands::FETCH_TODOS, (), Target::Auto));
            ctx.submit_command(Command::new(commands::FETCH_APPOINTMENTS, (), Target::Auto));
        } else if cmd.is(commands::FETCH_TODOS) {
            FetchTodosJob::new(ctx.get_external_handle(), self.providers.clone()).run();
        } else if cmd.is(commands::FETCH_APPOINTMENTS) {
            FetchAppointmentsJob::new(ctx.get_external_handle(), &data.config.calendar_config)
                .run();
        } else if let Some((provider, todos)) = cmd.get(commands::TODOS_FETCHED) {
            data.providers[*provider].items = todos.clone();
        } else if let Some(todo) = cmd.get(commands::OPEN_TODO) {
            data.navigation = Navigation::Selected(todo.clone());
        } else if cmd.get(commands::CLOSE_TODO).is_some() {
            data.navigation = Navigation::List;
        } else if let Some(link) = cmd.get(commands::OPEN_LINK) {
            open::that_in_background(link);
        } else if let Some(appointment) = cmd.get(commands::NEXT_APPOINTMENT_FETCHED) {
            data.next_appointment = appointment.clone();
        } else if let Some(provider) = cmd.get(commands::TOGGLE_PROVIDER) {
            if let Some(index) = data.providers.iter().position(|p| p.name == provider.name) {
                data.providers[index].collapsed = !data.providers[index].collapsed;
            }
        } else {
            return Handled::No;
        }
        Handled::Yes
    }
}

impl SidenotesDelegate {
    fn map_providers(providers: &[(ProviderSettings, ProviderImpl)]) -> Vector<TodoProvider> {
        providers
            .iter()
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
}
