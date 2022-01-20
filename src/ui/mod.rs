use druid::widget::ViewSwitcher;
use druid::{LensExt, Widget, WidgetExt};

use views::detail::detail_builder;
use views::list::list_builder;

pub(crate) use self::delegate::SidenotesDelegate;
use crate::models::*;

pub mod commands;
mod delegate;
mod lazy_icon;
mod lens;
mod prism;
pub mod theme;
mod views;
mod widgets;

pub fn ui_builder() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |state: &AppState, _| state.navigation.clone(),
        |nav: &Navigation, _: &AppState, _| match nav {
            Navigation::List => list_builder().boxed(),
            Navigation::Selected(_) => detail_builder()
                .lens(AppState::navigation.then(Navigation::selected()))
                .boxed(),
        },
    )
}
