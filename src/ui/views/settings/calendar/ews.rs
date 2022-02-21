use crate::calendar::ews::EwsConfig;
use crate::calendar::CalendarConfig;
use crate::ui::prism::CalendarConfigPrism;
use crate::ui::views::settings::widgets::{
    CalendarSettingsBuilder, ProviderSettingsRow, SettingsBuilder,
};
use druid::Widget;
use druid_widget_nursery::prism::Prism;

pub fn ews_settings() -> SettingsBuilder<EwsConfig> {
    SettingsBuilder::new("EWS")
        .add_field(ProviderSettingsRow::new("URL", EwsConfig::url))
        .add_field(ProviderSettingsRow::new("Username", EwsConfig::username))
        .add_field(ProviderSettingsRow::new("Password", EwsConfig::password).secret())
}

pub fn view() -> impl Widget<EwsConfig> {
    ews_settings().build_view()
}

pub fn edit() -> impl Widget<EwsConfig> {
    ews_settings().build_edit()
}

impl Prism<CalendarConfig, EwsConfig> for CalendarConfigPrism {
    fn get(&self, data: &CalendarConfig) -> Option<EwsConfig> {
        #[allow(irrefutable_let_patterns)]
        if let CalendarConfig::Ews(config) = data {
            Some(config.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut CalendarConfig, inner: EwsConfig) {
        *data = CalendarConfig::Ews(inner);
    }
}
