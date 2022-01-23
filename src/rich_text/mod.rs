use druid::text::{RichText, RichTextBuilder};
use druid::Data;
use enum_dispatch::enum_dispatch;

#[cfg(feature = "p_jira")]
pub use self::jira_markup::*;
pub use self::markdown::*;

#[cfg(feature = "p_jira")]
mod jira_markup;
mod markdown;

#[enum_dispatch(RawRichText)]
pub trait IntoRichText {
    fn into_rich_text(self) -> RichText;
}

impl IntoRichText for String {
    fn into_rich_text(self) -> RichText {
        let mut builder = RichTextBuilder::new();
        builder.push(&self);

        builder.build()
    }
}

#[enum_dispatch]
#[derive(Clone, Debug, Data)]
pub enum RawRichText {
    Markdown,
    #[cfg(feature = "p_jira")]
    Jira(JiraMarkup),
}

pub fn get_font_size_for_heading(lvl: u32) -> f64 {
    match lvl {
        1 => 38.,
        2 => 32.0,
        3 => 26.0,
        4 => 20.0,
        5 => 16.0,
        _ => 12.0,
    }
}
