use crate::calendar::TZ;
use druid::widget::{Either, Flex, Label, LineBreaking, MainAxisAlignment, Maybe, Slider};
use druid::{Command, FontDescriptor, FontFamily, Insets, Target, Widget, WidgetExt};

use crate::models::{Appointment, Navigation};
use crate::ui::commands;
use crate::ui::lazy_icon::*;
use crate::ui::lens::{AppointmentProgress, TimeUntilNextAppointment};
use crate::ui::views::list::timer::TimerController;
use crate::ui::widgets::ClickableArea;
use crate::CARD_COLOR;

thread_local! {
    static CALENDAR_ICON: LazyIcon = LazyIcon::new(|| {
        include_str!("../../../../assets/icons/calendar.svg").load()
    });
}

fn meeting_progress_indicator() -> impl Widget<Appointment> {
    Slider::new()
        .expand_width()
        .disabled_if(|_, _| true)
        .lens(AppointmentProgress)
        .controller(TimerController::default())
}

fn meeting_time_indicator() -> impl Widget<Appointment> {
    let time_font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.0);
    let time_until = Label::new(|time_until: &String, _: &_| time_until.clone())
        .with_font(time_font)
        .with_text_color(druid::theme::PLACEHOLDER_COLOR)
        .align_left()
        .lens(TimeUntilNextAppointment)
        .controller(TimerController::default());

    Either::new(
        |appointment, _| {
            let now = TZ::now();
            let time_until = appointment.start_time - now;

            time_until.num_minutes() > 0
        },
        time_until,
        meeting_progress_indicator(),
    )
    .controller(TimerController::default())
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
        .controller(ClickableArea)
        .on_click(|event_ctx, _: &mut _, _: &_| {
            event_ctx.submit_command(Command::new(
                commands::NAVIGATE,
                Navigation::Appointments,
                Target::Auto,
            ))
        })
}
