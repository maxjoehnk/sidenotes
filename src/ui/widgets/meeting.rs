use druid::widget::{Flex, Label, MainAxisAlignment, Maybe};
use druid::{FontDescriptor, FontFamily, Widget, WidgetExt};

use crate::models::Appointment;
use crate::ui::lazy_icon::*;
use crate::ui::lens::TimeUntilNextAppointment;
use crate::ui::widgets::timer::TimerController;
use crate::CARD_COLOR;

thread_local! {
    static CALENDAR_ICON: LazyIcon = LazyIcon::new(|| {
        include_str!("../../../assets/icons/calendar.svg").load()
    });
}

fn meeting_body() -> Flex<Appointment> {
    let time_font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.0);
    Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .with_child(
            Label::new(|appointment: &Appointment, _: &_| appointment.title.clone()).align_left(),
        )
        .with_child(
            Label::new(|time_until: &String, _: &_| time_until.clone())
                .with_font(time_font)
                .with_text_color(druid::theme::PLACEHOLDER_COLOR)
                .align_left()
                .lens(TimeUntilNextAppointment)
                .controller(TimerController::default()),
        )
}

fn meeting_card() -> impl Widget<Appointment> {
    let icon = CALENDAR_ICON.to_svg().fix_size(48., 48.).padding(8.);

    let body = meeting_body();

    Flex::row()
        .must_fill_main_axis(true)
        .with_child(icon)
        .with_flex_child(body, 1.0)
        .fix_height(80.)
        .background(CARD_COLOR)
        .rounded(4.0)
        .padding(8.0)
}

pub fn meeting_builder() -> impl Widget<Option<Appointment>> {
    Maybe::new(meeting_card, Flex::row)
}
