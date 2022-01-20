use druid::widget::*;
use druid::Widget;

use self::meeting::meeting_builder;
use self::provider_item::provider_builder;
use crate::models::*;

mod meeting;
mod provider_item;
mod timer;
mod todo_item;

pub fn list_builder() -> impl Widget<AppState> {
    let list = List::new(provider_builder).lens(AppState::providers());
    let list_view = Flex::column()
        .with_child(meeting_builder().lens(AppState::next_appointment))
        .with_child(list);

    Scroll::new(list_view).vertical()
}
