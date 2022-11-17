use self::container::MarkupContainer;
use self::settings_wrapper::MarkupSettingsWrapper;
use crate::rich_text::{MarkupItem, RawRichText};
use crate::ui::lens::Markup;
use crate::ui::prism::{MarkupTablePrism, MarkupTextPrism};
use crate::ui::widgets::markup::table::MarkupTable;
use druid::text::RichText;
use druid::widget::{LineBreaking, List, RawLabel};
use druid::{Widget, WidgetExt};
use druid_widget_nursery::enum_switcher::Switcher;

mod container;
mod settings_wrapper;
mod table;

pub fn markup_builder() -> impl Widget<RawRichText> {
    let list = List::new(markup_item_builder).lens(Markup);

    MarkupSettingsWrapper::new(list)
}

fn markup_item_builder() -> impl Widget<MarkupItem> {
    let content = Switcher::new()
        .with_variant(MarkupTextPrism, markup_text_builder())
        .with_variant(MarkupTablePrism, MarkupTable::new())
        .lens(MarkupItem::part);

    MarkupContainer::new(content).expand_width()
}

fn markup_text_builder() -> impl Widget<RichText> {
    RawLabel::new().with_line_break_mode(LineBreaking::WordWrap)
}
