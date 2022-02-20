use druid::Widget;
use druid_widget_nursery::enum_switcher::Switcher;

use views::detail::detail_builder;
use views::list::list_builder;
use views::settings::calendar::calendar_settings_builder;
use views::settings::global::global_settings_builder;
use views::settings::providers::{edit_provider, new_provider_selector, provider_settings_builder};
use views::settings::settings_builder;

pub(crate) use self::delegate::SidenotesDelegate;
use crate::models::*;
use crate::ui::prism::*;

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
        .with_variant(NavigationSettingsPrism, settings_builder())
        .with_variant(NavigationGlobalSettingsPrism, global_settings_builder())
        .with_variant(NavigationProviderSettingsPrism, provider_settings_builder())
        .with_variant(NavigationCalendarSettingsPrism, calendar_settings_builder())
        .with_variant(NavigationEditProviderPrism, edit_provider())
        .with_variant(NavigationNewProviderPrism, new_provider_selector())
}
