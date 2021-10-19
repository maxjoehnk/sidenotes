use crate::rich_text::IntoRichText;
use druid::text::{RichText, RichTextBuilder};
use serde::Deserialize;
use druid::Data;

#[repr(transparent)]
#[derive(Debug, Clone, Deserialize, Data)]
pub struct JiraMarkup(String);

impl From<String> for JiraMarkup {
    fn from(text: String) -> Self {
        Self(text)
    }
}

impl IntoRichText for JiraMarkup {
    fn into_rich_text(self) -> RichText {
        // TODO: implement jira parser
        let mut builder = RichTextBuilder::new();
        builder.push(&self.0);

        builder.build()
    }
}
