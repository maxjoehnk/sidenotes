use druid::widget::{Controller, ViewSwitcher};
use druid::{Env, Event, EventCtx, LensExt, Widget, WidgetExt};

use widgets::detail::detail_builder;
use widgets::list::list_builder;

use crate::models::*;

pub mod commands;
mod lens;
mod prism;
pub mod theme;
mod widgets;

struct Sidenotes;

impl<W: Widget<AppState>> Controller<AppState, W> for Sidenotes {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if let Event::Command(cmd) = event {
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
            } else if let Some(provider) = cmd.get(commands::TOGGLE_PROVIDER) {
                if let Some(index) = data.providers.iter().position(|p| p.name == provider.name) {
                    data.providers[index].collapsed = !data.providers[index].collapsed;
                }
            }
        } else {
            child.event(ctx, event, data, env)
        }
    }
}

pub fn ui_builder() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |state: &AppState, _| state.navigation.clone(),
        |nav: &Navigation, _: &AppState, _| match nav {
            Navigation::List => list_builder().lens(AppState::providers()).boxed(),
            Navigation::Selected(_) => detail_builder()
                .lens(AppState::navigation.then(Navigation::selected()))
                .boxed(),
        },
    )
    .controller(Sidenotes)
}
