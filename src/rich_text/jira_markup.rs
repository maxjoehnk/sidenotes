use crate::rich_text::{get_font_size_for_heading, IntoRichText};
use crate::ui::commands::OPEN_LINK;
use crate::LINK_COLOR;
use druid::text::{Attribute, AttributesAdder, RichText, RichTextBuilder};
use druid::{Color, Data, FontFamily, FontStyle, FontWeight};
use jira_parser::ast::*;
use serde::Deserialize;

const BLOCKQUOTE_COLOR: Color = Color::grey8(0x88);
const INSERTED_COLOR: Color = Color::rgb8(0, 255, 0);
const DELETED_COLOR: Color = Color::rgb8(255, 0, 0);

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
        let mut builder = RichTextBuilder::new();
        let tags = jira_parser::parse(&self.0).unwrap();
        build_tags(&mut builder, tags, vec![]);

        builder.build()
    }
}

fn build_tags(builder: &mut RichTextBuilder, tags: Vec<Tag>, attr: Vec<Attribute>) {
    for tag in tags {
        match tag {
            Tag::Text(text) => builder.push(&text).add_attributes(&attr),
            Tag::Strong(text) => builder
                .push(&text)
                .weight(FontWeight::BOLD)
                .add_attributes(&attr),
            Tag::Emphasis(text) => builder
                .push(&text)
                .style(FontStyle::Italic)
                .add_attributes(&attr),
            Tag::Panel(panel) => {
                if let Some(title) = panel.title {
                    builder.push(&title).size(26.);
                    builder.push("\n");
                }
                build_tags(builder, panel.content, attr.clone());
                builder.push("\n");
            }
            Tag::Newline => {
                builder.push("\n");
            }
            Tag::InlineQuote(text) => builder
                .push(&text)
                .style(FontStyle::Italic)
                .text_color(BLOCKQUOTE_COLOR)
                .add_attributes(&attr),
            Tag::Quote(text) => builder
                .push(&text)
                .style(FontStyle::Italic)
                .text_color(BLOCKQUOTE_COLOR)
                .add_attributes(&attr),
            Tag::Monospaced(text) => builder
                .push(&text)
                .font_family(FontFamily::MONOSPACE)
                .add_attributes(&attr),
            Tag::Inserted(text) => builder
                .push(&text)
                .underline(true)
                .text_color(INSERTED_COLOR)
                .add_attributes(&attr),
            Tag::Deleted(text) => builder
                .push(&text)
                .strikethrough(true)
                .text_color(DELETED_COLOR)
                .add_attributes(&attr),
            Tag::Subscript(text) => builder.push(&text).add_attributes(&attr),
            Tag::Superscript(text) => builder.push(&text).add_attributes(&attr),
            Tag::Color(color, text) => builder
                .push(&text)
                .text_color(from_color(&color))
                .add_attributes(&attr),
            Tag::Heading(level, content) => build_tags(
                builder,
                content,
                vec![Attribute::FontSize(
                    get_font_size_for_heading(level as u32).into(),
                )],
            ),
            Tag::UnorderedList(items) => {
                for item in items {
                    for _ in 0..item.level {
                        builder.push("  ");
                    }
                    builder.push("* ");
                    build_tags(builder, item.content, attr.clone());
                    builder.push("\n");
                }
            }
            Tag::Link(text, link) => builder
                .push(&text)
                .underline(true)
                .text_color(LINK_COLOR)
                .link(OPEN_LINK.with(link))
                .add_attributes(&attr),
            _ => {}
        }
    }
}

fn from_color(color: &str) -> Color {
    match color {
        "red" => Color::RED,
        "green" => Color::GREEN,
        "blue" => Color::BLUE,
        c => Color::from_hex_str(c).unwrap_or(Color::WHITE),
    }
}

trait AttributesAdderExt {
    fn add_attributes(&mut self, attributes: &[Attribute]);
}

impl<'a> AttributesAdderExt for AttributesAdder<'a> {
    fn add_attributes(&mut self, attributes: &[Attribute]) {
        for attr in attributes {
            self.add_attr(attr.clone());
        }
    }
}
