use crate::models::AppState;
use crate::ui::widgets::header_builder;
use druid::widget::Flex;
use druid::Widget;

pub fn calendar_settings_builder() -> impl Widget<AppState> {
    let header = header_builder("Calendar");

    Flex::column().with_child(header)
}
