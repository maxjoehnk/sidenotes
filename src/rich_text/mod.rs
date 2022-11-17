use derive_more::From;
use druid::text::{RichText, RichTextBuilder};
use druid::{Data, Lens};
use enum_dispatch::enum_dispatch;
use im::{vector, Vector};

#[cfg(feature = "jira")]
pub use self::jira_markup::*;
pub use self::markdown::*;

#[cfg(feature = "jira")]
mod jira_markup;
mod markdown;

#[enum_dispatch(RawRichText)]
pub trait IntoMarkup {
    fn into_markup(self, disable_colorized_backgrounds: bool) -> Vector<MarkupItem>;
}

impl IntoMarkup for String {
    fn into_markup(self, _: bool) -> Vector<MarkupItem> {
        let mut builder = RichTextBuilder::new();
        builder.push(&self);

        let text = builder.build();

        vector![MarkupItem {
            part: text.into(),
            style: Default::default(),
        }]
    }
}

#[enum_dispatch]
#[derive(Clone, Debug, Data)]
pub enum RawRichText {
    Markdown,
    #[cfg(feature = "jira")]
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

#[derive(Debug, Clone, Data, Lens)]
pub struct MarkupItem {
    pub part: MarkupPart,
    pub style: MarkupItemStyle,
}

#[derive(Debug, Default, Clone, Data, Lens)]
pub struct MarkupItemStyle {
    pub background: Option<String>,
}

#[derive(Debug, Clone, From, Data)]
pub enum MarkupPart {
    Text(RichText),
    Table(Table),
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Table {
    pub rows: Vector<TableRow>,
}

#[derive(Debug, Clone, Data, Lens)]
pub struct TableRow {
    pub fields: Vector<TableField>,
}

#[derive(Debug, Clone, Data, Lens)]
pub struct TableField {
    pub content: RichText,
    pub is_heading: bool,
}
