use druid::Widget;
use druid_widget_nursery::enum_switcher::Switcher;

use views::detail::detail_builder;
use views::list::list_builder;

pub(crate) use self::delegate::SidenotesDelegate;
use crate::models::*;
use crate::ui::prism::{NavigationListPrism, NavigationSelectedPrism};

pub mod commands;
mod delegate;
mod lazy_icon;
mod lens;
mod prism;
pub mod theme;
mod views;
mod widgets;

pub fn ui_builder() -> impl Widget<AppState> {
    Switcher::new()
        .with_variant(NavigationListPrism, list_builder())
        .with_variant(NavigationSelectedPrism, detail_builder())
}
