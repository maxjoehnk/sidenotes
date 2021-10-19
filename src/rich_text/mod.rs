use druid::text::{RichText, RichTextBuilder};
use druid::Data;
use enum_dispatch::enum_dispatch;

pub use self::markdown::*;
#[cfg(feature = "jira")]
pub use self::jira_markup::*;

mod markdown;
#[cfg(feature = "jira")]
mod jira_markup;

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
    #[cfg(feature = "jira")]
    Jira(JiraMarkup)
}
