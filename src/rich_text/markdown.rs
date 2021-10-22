use druid::text::RichText;
use druid::Data;
use serde::Deserialize;

use crate::rich_text::IntoRichText;

#[derive(Debug, Clone, Deserialize, Data)]
#[repr(transparent)]
pub struct Markdown(pub String);

impl From<String> for Markdown {
    fn from(text: String) -> Self {
        Self(text)
    }
}

impl IntoRichText for Markdown {
    fn into_rich_text(self) -> RichText {
        self::converter::render_text(&self.0)
    }
}

// TODO: replace with own implementation with better support for all markdown features
// Source: https://github.com/linebender/druid/blob/master/druid/examples/markdown_preview.rs
mod converter {
    // Copyright 2020 The Druid Authors.
    //
    // Licensed under the Apache License, Version 2.0 (the "License");
    // you may not use this file except in compliance with the License.
    // You may obtain a copy of the License at
    //
    //     http://www.apache.org/licenses/LICENSE-2.0
    //
    // Unless required by applicable law or agreed to in writing, software
    // distributed under the License is distributed on an "AS IS" BASIS,
    // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    // See the License for the specific language governing permissions and
    // limitations under the License.
    use druid::text::{AttributesAdder, RichText, RichTextBuilder};
    use druid::{Color, FontFamily, FontStyle, FontWeight};
    use pulldown_cmark::{Event as ParseEvent, Options, Parser, Tag};

    use crate::rich_text::get_font_size_for_heading;
    use crate::ui::commands::OPEN_LINK;

    const BLOCKQUOTE_COLOR: Color = Color::grey8(0x88);
    const LINK_COLOR: Color = Color::rgb8(0, 0, 0xEE);

    pub fn render_text(text: &str) -> RichText {
        let mut current_pos = 0;
        let mut builder = RichTextBuilder::new();
        let mut tag_stack = Vec::new();

        let parser = Parser::new_ext(text, Options::ENABLE_STRIKETHROUGH);
        for event in parser {
            match event {
                ParseEvent::Start(tag) => {
                    tag_stack.push((current_pos, tag));
                }
                ParseEvent::Text(txt) => {
                    builder.push(&txt);
                    current_pos += txt.len();
                }
                ParseEvent::End(end_tag) => {
                    let (start_off, tag) = tag_stack
                        .pop()
                        .expect("parser does not return unbalanced tags");
                    assert_eq!(end_tag, tag, "mismatched tags?");
                    add_attribute_for_tag(
                        &tag,
                        builder.add_attributes_for_range(start_off..current_pos),
                    );
                    if add_newline_after_tag(&tag) {
                        builder.push("\n\n");
                        current_pos += 2;
                    }
                }
                ParseEvent::Code(txt) => {
                    builder.push(&txt).font_family(FontFamily::MONOSPACE);
                    current_pos += txt.len();
                }
                ParseEvent::Html(txt) => {
                    builder
                        .push(&txt)
                        .font_family(FontFamily::MONOSPACE)
                        .text_color(BLOCKQUOTE_COLOR);
                    current_pos += txt.len();
                }
                ParseEvent::HardBreak => {
                    builder.push("\n\n");
                    current_pos += 2;
                }
                _ => (),
            }
        }
        builder.build()
    }

    fn add_newline_after_tag(tag: &Tag) -> bool {
        !matches!(
            tag,
            Tag::Emphasis | Tag::Strong | Tag::Strikethrough | Tag::Link(..)
        )
    }

    fn add_attribute_for_tag(tag: &Tag, mut attrs: AttributesAdder) {
        match tag {
            Tag::Heading(lvl) => {
                let font_size = get_font_size_for_heading(*lvl);
                attrs.size(font_size).weight(FontWeight::BOLD);
            }
            Tag::BlockQuote => {
                attrs.style(FontStyle::Italic).text_color(BLOCKQUOTE_COLOR);
            }
            Tag::CodeBlock(_) => {
                attrs.font_family(FontFamily::MONOSPACE);
            }
            Tag::Emphasis => {
                attrs.style(FontStyle::Italic);
            }
            Tag::Strong => {
                attrs.weight(FontWeight::BOLD);
            }
            Tag::Strikethrough => {
                attrs.strikethrough(true);
            }
            Tag::Link(_link_ty, target, _title) => {
                attrs
                    .underline(true)
                    .text_color(LINK_COLOR)
                    .link(OPEN_LINK.with(target.to_string()));
            }
            // ignore other tags for now
            _ => (),
        }
    }
}
