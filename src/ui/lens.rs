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
