use crate::models::{AppState, Appointment};
use crate::ui::widgets::header_builder;
use crate::CARD_COLOR;
use chrono::Timelike;
use druid::widget::*;
use druid::{FontDescriptor, FontFamily, Insets, Widget};

fn meeting_time_indicator() -> impl Widget<Appointment> {
    let time_font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.0);

    Label::new(|appointment: &Appointment, _: &_| {
        format!(
            "{}:{:0<2} - {}:{:0<2}",
            appointment.start_time.hour(),
            appointment.start_time.minute(),
            appointment.end_time.hour(),
            appointment.end_time.minute()
        )
    })
    .with_font(time_font)
    .with_text_color(druid::theme::PLACEHOLDER_COLOR)
    .align_left()
}

fn meeting_body() -> Flex<Appointment> {
    Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .with_child(
            Label::new(|appointment: &Appointment, _: &_| appointment.title.clone())
                .with_line_break_mode(LineBreaking::WordWrap)
                .align_left()
                .padding(Insets::new(0., 0., 8., 0.)),
        )
        .with_child(meeting_time_indicator())
}

fn meeting_card() -> impl Widget<Appointment> {
    let body = meeting_body();

    Flex::row()
        .must_fill_main_axis(true)
        .with_flex_child(body, 1.0)
        .padding(8.0)
        .background(CARD_COLOR)
        .rounded(4.0)
        .padding(4.0)
}

pub fn appointments_builder() -> impl Widget<AppState> {
    let header = header_builder(|_: &_, _: &_| "Appointments".to_string());
    let list_view = List::new(meeting_card).lens(AppState::appointments);
    let list_view = Scroll::new(list_view).vertical().expand_height();

    Flex::column()
        .with_child(header)
        .with_flex_child(list_view, 1.0)
        .must_fill_main_axis(true)
}
