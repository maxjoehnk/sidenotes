use druid::{FontDescriptor, FontFamily, Widget, WidgetExt};
use druid::widget::{Flex, Label, MainAxisAlignment, Svg, SvgData};
use druid_widget_nursery::prism::DisablePrismWrap;

use crate::{CARD_COLOR, MEETING_TIME_COLOR};
use crate::calendar::TZ;
use crate::models::{Appointment, AppState};
use crate::ui::lens::TimeUntilNextAppointment;
use crate::ui::prism::NextAppointment;
use crate::ui::widgets::timer::TimerController;

const CALENDAR_ICON: &str = include_str!("../../../assets/icons/calendar.svg");

fn meeting_body() -> Flex<Appointment> {
    let time_font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.0);
    Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .with_child(Label::new(|appointment: &Appointment, _: &_| appointment.title.clone()).align_left())
        .with_child(
            Label::new(|time_until: &String, _: &_| time_until.clone())
                .with_font(time_font)
                .with_text_color(MEETING_TIME_COLOR)
                .align_left()
                .lens(TimeUntilNextAppointment)
                .controller(TimerController::default())
        )
}

fn meeting_card() -> impl Widget<Appointment> {
    let icon = CALENDAR_ICON.parse::<SvgData>().unwrap();
    let icon = Svg::new(icon).fix_size(48., 48.).padding(8.);

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

pub fn meeting_builder() -> impl Widget<AppState> {
    DisablePrismWrap::new(meeting_card(), Appointment {
        title: Default::default(),
        description: Default::default(),
        meeting_link: None,
        end_time: TZ::now(),
        start_time: TZ::now(),
    }, NextAppointment)
}
