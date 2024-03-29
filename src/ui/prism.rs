use crate::calendar::CalendarConfig;
use crate::config::Config;
use druid::text::RichText;
use druid_widget_nursery::prism::Prism;

use crate::models::*;
use crate::providers::ProviderConfigEntry;
use crate::rich_text::{MarkupPart, Table};

pub struct ProviderConfigPrism;
pub struct CalendarConfigPrism;

pub struct NavigationListPrism;
impl Prism<AppState, AppState> for NavigationListPrism {
    fn get(&self, data: &AppState) -> Option<AppState> {
        if matches!(data.navigation, Navigation::List) {
            Some(data.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: AppState) {
        if matches!(data.navigation, Navigation::List) {
            *data = inner;
        }
    }
}

pub struct NavigationSelectedPrism;
impl Prism<AppState, Todo> for NavigationSelectedPrism {
    fn get(&self, data: &AppState) -> Option<Todo> {
        if let Navigation::Selected(ref todo) = data.navigation {
            Some(todo.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: Todo) {
        if let Navigation::Selected(ref mut todo) = data.navigation {
            *todo = inner;
        }
    }
}

pub struct NavigationSettingsPrism;
impl Prism<AppState, AppState> for NavigationSettingsPrism {
    fn get(&self, data: &AppState) -> Option<AppState> {
        if matches!(data.navigation, Navigation::Settings) {
            Some(data.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: AppState) {
        if matches!(data.navigation, Navigation::Settings) {
            *data = inner;
        }
    }
}

pub struct NavigationProviderSettingsPrism;
impl Prism<AppState, AppState> for NavigationProviderSettingsPrism {
    fn get(&self, data: &AppState) -> Option<AppState> {
        if matches!(data.navigation, Navigation::ProviderSettings) {
            Some(data.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: AppState) {
        if matches!(data.navigation, Navigation::ProviderSettings) {
            *data = inner;
        }
    }
}

pub struct NavigationGlobalSettingsPrism;
impl Prism<AppState, Config> for NavigationGlobalSettingsPrism {
    fn get(&self, data: &AppState) -> Option<Config> {
        if let Navigation::GlobalSettings(config) = &data.navigation {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: Config) {
        if let Navigation::GlobalSettings(config) = &mut data.navigation {
            *config = inner;
        }
    }
}

pub struct NavigationCalendarSettingsPrism;
impl Prism<AppState, AppState> for NavigationCalendarSettingsPrism {
    fn get(&self, data: &AppState) -> Option<AppState> {
        if matches!(data.navigation, Navigation::CalendarSettings) {
            Some(data.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: AppState) {
        if matches!(data.navigation, Navigation::CalendarSettings) {
            *data = inner;
        }
    }
}

pub struct NavigationEditProviderPrism;
impl Prism<AppState, ProviderConfigEntry> for NavigationEditProviderPrism {
    fn get(&self, data: &AppState) -> Option<ProviderConfigEntry> {
        if let Navigation::EditProvider(ref config) = data.navigation {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: ProviderConfigEntry) {
        if let Navigation::EditProvider(ref mut config) = data.navigation {
            config.provider = inner.provider;
            config.settings = inner.settings
        }
    }
}

pub struct NavigationNewProviderPrism;
impl Prism<AppState, AppState> for NavigationNewProviderPrism {
    fn get(&self, data: &AppState) -> Option<AppState> {
        if matches!(data.navigation, Navigation::NewProvider) {
            Some(data.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: AppState) {
        if matches!(data.navigation, Navigation::NewProvider) {
            *data = inner;
        }
    }
}

pub struct NavigationEditCalendarPrism;
impl Prism<AppState, CalendarConfig> for NavigationEditCalendarPrism {
    fn get(&self, data: &AppState) -> Option<CalendarConfig> {
        if let Navigation::EditCalendar((_, ref config)) = data.navigation {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: CalendarConfig) {
        if let Navigation::EditCalendar((_, ref mut config)) = data.navigation {
            *config = inner;
        }
    }
}

pub struct NavigationNewCalendarPrism;
impl Prism<AppState, AppState> for NavigationNewCalendarPrism {
    fn get(&self, data: &AppState) -> Option<AppState> {
        if matches!(data.navigation, Navigation::NewCalendar) {
            Some(data.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: AppState) {
        if matches!(data.navigation, Navigation::NewCalendar) {
            *data = inner;
        }
    }
}

pub struct NavigationAppointmentsPrism;
impl Prism<AppState, AppState> for NavigationAppointmentsPrism {
    fn get(&self, data: &AppState) -> Option<AppState> {
        if matches!(data.navigation, Navigation::Appointments) {
            Some(data.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: AppState) {
        if matches!(data.navigation, Navigation::Appointments) {
            *data = inner;
        }
    }
}

pub struct MarkupTextPrism;
impl Prism<MarkupPart, RichText> for MarkupTextPrism {
    fn get(&self, data: &MarkupPart) -> Option<RichText> {
        if let MarkupPart::Text(text) = data {
            Some(text.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut MarkupPart, inner: RichText) {
        *data = MarkupPart::Text(inner);
    }
}

pub struct MarkupTablePrism;
impl Prism<MarkupPart, Table> for MarkupTablePrism {
    fn get(&self, data: &MarkupPart) -> Option<Table> {
        if let MarkupPart::Table(table) = data {
            Some(table.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut MarkupPart, inner: Table) {
        *data = MarkupPart::Table(inner);
    }
}
