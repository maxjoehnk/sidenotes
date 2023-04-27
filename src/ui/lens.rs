use crate::calendar::TZ;
use crate::models::Appointment;
use crate::rich_text::{IntoMarkup, MarkupItem, RawRichText};
use chrono::{DateTime, Timelike};
use druid::text::{RichText, RichTextBuilder};
use druid::Lens;
use im::Vector;

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
        let time = format!("{minutes_until} minutes");

        if time_until.num_minutes() > 0 {
            format!("In {time}")
        } else {
            format!("Since {time}")
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

pub struct Markup;

impl Lens<(RawRichText, bool), Vector<MarkupItem>> for Markup {
    fn with<V, F: FnOnce(&Vector<MarkupItem>) -> V>(&self, data: &(RawRichText, bool), f: F) -> V {
        let markup = data.0.clone().into_markup(data.1);
        f(&markup)
    }

    fn with_mut<V, F: FnOnce(&mut Vector<MarkupItem>) -> V>(
        &self,
        data: &mut (RawRichText, bool),
        f: F,
    ) -> V {
        let mut markup = data.0.clone().into_markup(data.1);
        f(&mut markup)
    }
}
