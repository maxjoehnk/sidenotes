use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};

use crate::models::{AppState, Navigation};
use crate::ui::commands;

pub(crate) struct SidenotesDelegate;

impl AppDelegate<AppState> for SidenotesDelegate {
    fn command(
        &mut self,
        _: &mut DelegateCtx,
        _: Target,
        cmd: &Command,
        data: &mut AppState,
        _: &Env,
    ) -> Handled {
        if let Some(config) = cmd.get(commands::UI_CONFIG_LOADED) {
            data.ui_config = *config;
        } else if let Some(providers) = cmd.get(commands::PROVIDERS_CONFIGURED) {
            data.providers = providers.clone();
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
