use std::str::FromStr;

use druid::text::{Attribute, AttributesAdder, RichTextBuilder};
use druid::{Color, Data, FontFamily, FontStyle, FontWeight};
use im::Vector;
use palette::{RelativeContrast, Srgb};
use serde::Deserialize;

use jira_parser::ast::{self, *};

use crate::rich_text::{
    get_font_size_for_heading, IntoMarkup, MarkupItem, MarkupItemStyle, MarkupPart, Table,
    TableField, TableRow,
};
use crate::ui::commands::OPEN_LINK;
use crate::LINK_COLOR;

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

impl IntoMarkup for JiraMarkup {
    fn into_markup(self, disable_colorized_backgrounds: bool) -> Vector<MarkupItem> {
        let tags = jira_parser::parse(&self.0).unwrap_or_default();
        let attrs = vec![];

        tags.into_iter()
            .fold(Vec::<MarkupItemBuilder>::new(), |mut items, tag| {
                match (items.pop(), tag) {
                    (last_item, Tag::Panel(panel)) => {
                        if let Some(last_item) = last_item {
                            items.push(last_item);
                        }
                        items.push(build_panel_title(&panel, disable_colorized_backgrounds));
                        items.push(build_panel_content(panel, disable_colorized_backgrounds));
                    }
                    (last_item, Tag::Table(table)) => {
                        if let Some(last_item) = last_item {
                            items.push(last_item);
                        }
                        items.push(build_table(table));
                    }
                    (Some(MarkupItemBuilder::Text(mut builder, style)), tag) => {
                        build_tag(&mut builder, &attrs, tag);
                        items.push(MarkupItemBuilder::Text(builder, style));
                    }
                    (last_item, tag) => {
                        if let Some(last_item) = last_item {
                            items.push(last_item);
                        }
                        let mut builder = RichTextBuilder::new();
                        build_tag(&mut builder, &attrs, tag);
                        items.push(MarkupItemBuilder::Text(
                            Box::new(builder),
                            Default::default(),
                        ));
                    }
                }

                items
            })
            .into_iter()
            .map(MarkupItem::from)
            .collect()
    }
}

fn build_panel_title(panel: &Panel, disable_colorized_backgrounds: bool) -> MarkupItemBuilder {
    let style = MarkupItemStyle {
        background: (!disable_colorized_backgrounds)
            .then_some(())
            .and_then(|_| panel.title_background_color.clone()),
    };
    let mut text_builder = RichTextBuilder::new();
    if let Some(ref title) = panel.title {
        let mut title_builder = text_builder.push(title);
        title_builder.size(26.);
        if let Some(color) = get_high_contrast_text_color(&style) {
            title_builder.text_color(color);
        }
    }

    MarkupItemBuilder::Text(Box::new(text_builder), style)
}

fn build_panel_content(panel: Panel, disable_colorized_backgrounds: bool) -> MarkupItemBuilder {
    let mut attrs = vec![];
    let style = MarkupItemStyle {
        background: (!disable_colorized_backgrounds)
            .then_some(())
            .and_then(|_| panel.background_color.clone()),
    };
    if let Some(color) = get_high_contrast_text_color(&style) {
        attrs.push(Attribute::TextColor(color.into()));
    }

    let mut builder = RichTextBuilder::new();
    for tag in panel.content.into_iter() {
        build_tag(&mut builder, &attrs, tag);
    }

    MarkupItemBuilder::Text(Box::new(builder), style)
}

fn build_table(table: ast::Table) -> MarkupItemBuilder {
    let attrs = vec![];
    let rows = table
        .rows
        .into_iter()
        .map(|row| TableRow {
            fields: row
                .0
                .into_iter()
                .map(|field| {
                    let (tags, is_heading) = match field {
                        ast::TableField::Plain(tags) => (tags, false),
                        ast::TableField::Heading(tags) => (tags, true),
                    };

                    let mut builder = RichTextBuilder::new();
                    for tag in tags.into_iter() {
                        build_tag(&mut builder, &attrs, tag);
                    }

                    TableField {
                        content: builder.build(),
                        is_heading,
                    }
                })
                .collect(),
        })
        .collect();

    MarkupItemBuilder::Table(Table { rows })
}

fn get_high_contrast_text_color(style: &MarkupItemStyle) -> Option<Color> {
    style
        .background
        .as_ref()
        .and_then(|background| Srgb::from_str(background).ok())
        .map(|background| {
            let white = Srgb::new(1., 1., 1.);
            if background
                .into_format::<f64>()
                .has_enhanced_contrast_text(&white)
            {
                Color::grey(1.)
            } else {
                Color::grey(0.)
            }
        })
}

enum MarkupItemBuilder {
    Text(Box<RichTextBuilder>, MarkupItemStyle),
    Table(Table),
}

impl From<MarkupItemBuilder> for MarkupItem {
    fn from(builder: MarkupItemBuilder) -> Self {
        match builder {
            MarkupItemBuilder::Text(text_builder, style) => MarkupItem {
                part: MarkupPart::Text(text_builder.build()),
                style,
            },
            MarkupItemBuilder::Table(table) => MarkupItem {
                part: MarkupPart::Table(table),
                style: MarkupItemStyle::default(),
            },
        }
    }
}

fn build_tags(builder: &mut RichTextBuilder, tags: Vec<Tag>, attr: Vec<Attribute>) {
    for tag in tags {
        build_tag(builder, &attr, tag)
    }
}

fn build_tag(builder: &mut RichTextBuilder, attr: &[Attribute], tag: Tag) {
    match tag {
        Tag::Text(text) => builder.push(&text).add_attributes(attr).close(),
        Tag::Strong(text) => builder
            .push(&text)
            .add_attributes(attr)
            .weight(FontWeight::BOLD)
            .close(),
        Tag::Emphasis(text) => builder
            .push(&text)
            .add_attributes(attr)
            .style(FontStyle::Italic)
            .close(),
        Tag::Newline => {
            builder.push("\n");
        }
        Tag::InlineQuote(text) => builder
            .push(&text)
            .add_attributes(attr)
            .style(FontStyle::Italic)
            .text_color(BLOCKQUOTE_COLOR)
            .close(),
        Tag::Quote(text) => builder
            .push(&text)
            .add_attributes(attr)
            .style(FontStyle::Italic)
            .text_color(BLOCKQUOTE_COLOR)
            .close(),
        Tag::Monospaced(text) => builder
            .push(&text)
            .add_attributes(attr)
            .font_family(FontFamily::MONOSPACE)
            .close(),
        Tag::Inserted(text) => builder
            .push(&text)
            .add_attributes(attr)
            .underline(true)
            .text_color(INSERTED_COLOR)
            .close(),
        Tag::Deleted(text) => builder
            .push(&text)
            .add_attributes(attr)
            .strikethrough(true)
            .text_color(DELETED_COLOR)
            .close(),
        Tag::Subscript(text) => builder.push(&text).add_attributes(attr).close(),
        Tag::Superscript(text) => builder.push(&text).add_attributes(attr).close(),
        Tag::Color(color, text) => builder
            .push(&text)
            .add_attributes(attr)
            .text_color(from_color(&color))
            .close(),
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
                build_tags(builder, item.content, attr.to_owned());
                builder.push("\n");
            }
        }
        Tag::Link(text, link) => builder
            .push(&text)
            .add_attributes(attr)
            .underline(true)
            .text_color(LINK_COLOR)
            .link(OPEN_LINK.with(link))
            .close(),
        _ => {}
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
    fn add_attributes(&mut self, attributes: &[Attribute]) -> &mut Self;
    fn close(&mut self);
}

impl<'a> AttributesAdderExt for AttributesAdder<'a> {
    fn add_attributes(&mut self, attributes: &[Attribute]) -> &mut Self {
        for attr in attributes {
            self.add_attr(attr.clone());
        }
        self
    }

    fn close(&mut self) {}
}
