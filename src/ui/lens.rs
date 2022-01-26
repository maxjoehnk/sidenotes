use crate::calendar::TZ;
use crate::models::{Appointment, TodoComment};
use crate::rich_text::IntoRichText;
use chrono::{DateTime, Timelike};
use druid::text::{RichText, RichTextBuilder};
use druid::Lens;

use crate::ui::commands::OPEN_LINK;
use crate::ui::theme::LINK_COLOR;

pub struct Link;

impl Lens<String, RichText> for Link {
    fn with<V, F: FnOnce(&RichText) -> V>(&self, data: &String, f: F) -> V {
        let mut builder = RichTextBuilder::new();
        builder
            .push(data)
            .underline(true)
            .text_color(LINK_COLOR)
            .link(OPEN_LINK.with(data.clone()));

        f(&builder.build())
    }

    fn with_mut<V, F: FnOnce(&mut RichText) -> V>(&self, data: &mut String, f: F) -> V {
        let mut builder = RichTextBuilder::new();
        builder
            .push(data)
            .underline(true)
            .text_color(LINK_COLOR)
            .link(OPEN_LINK.with(data.clone()));

        f(&mut builder.build())
    }
}

pub struct TimeUntilNextAppointment;

impl Lens<Appointment, String> for TimeUntilNextAppointment {
    fn with<V, F: FnOnce(&String) -> V>(&self, appointment: &Appointment, f: F) -> V {
        let time_until = Self::calculate(appointment.start_time);

        f(&time_until)
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, appointment: &mut Appointment, f: F) -> V {
        let mut time_until = Self::calculate(appointment.start_time);

        f(&mut time_until)
    }
}

impl TimeUntilNextAppointment {
    fn calculate(time: DateTime<TZ>) -> String {
        let now = TZ::now();
        let time_until = time - now;
        let minutes_until = time_until.num_minutes().abs();
        if minutes_until >= 60 {
            return format!("At {}:{:0<2}", time.hour(), time.minute());
        }
        let time = format!("{} minutes", minutes_until);

        if time_until.num_minutes() > 0 {
            format!("In {}", time)
        } else {
            format!("Since {}", time)
        }
    }
}

pub struct AppointmentProgress;

impl Lens<Appointment, f64> for AppointmentProgress {
    fn with<V, F: FnOnce(&f64) -> V>(&self, data: &Appointment, f: F) -> V {
        let start_time = data.start_time;
        let end_time = data.end_time;
        let now = TZ::now();
        let duration = end_time - start_time;
        let duration = duration.num_minutes() as f64;
        let passed = now - start_time;
        let passed = passed.num_minutes() as f64;
        let progress = passed / duration;

        f(&progress)
    }

    fn with_mut<V, F: FnOnce(&mut f64) -> V>(&self, data: &mut Appointment, f: F) -> V {
        let start_time = data.start_time;
        let end_time = data.end_time;
        let now = TZ::now();
        let duration = end_time - start_time;
        let duration = duration.num_minutes() as f64;
        let passed = now - start_time;
        let passed = passed.num_minutes() as f64;
        let mut progress = passed / duration;

        f(&mut progress)
    }
}

pub struct TodoCommentBody;

impl Lens<TodoComment, RichText> for TodoCommentBody {
    fn with<V, F: FnOnce(&RichText) -> V>(&self, comment: &TodoComment, f: F) -> V {
        f(&comment.text.clone().into_rich_text())
    }

    fn with_mut<V, F: FnOnce(&mut RichText) -> V>(&self, comment: &mut TodoComment, f: F) -> V {
        f(&mut comment.text.clone().into_rich_text())
    }
}
