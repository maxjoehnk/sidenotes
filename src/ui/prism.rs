use druid::text::RichText;
use druid_widget_nursery::prism::Prism;

use crate::models::*;
use crate::rich_text::IntoRichText;

pub struct TodoLink;

impl Prism<Todo, String> for TodoLink {
    fn get(&self, data: &Todo) -> Option<String> {
        data.link.clone()
    }

    fn put(&self, data: &mut Todo, inner: String) {
        data.link = Some(inner);
    }
}

pub struct TodoBody;

impl Prism<Todo, RichText> for TodoBody {
    fn get(&self, data: &Todo) -> Option<RichText> {
        data.body.clone().map(|body| body.into_rich_text())
    }

    fn put(&self, _: &mut Todo, _: RichText) {
        // Formatted body is readonly
    }
}
