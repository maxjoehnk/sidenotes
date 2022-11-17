use super::ClickableArea;
use crate::CARD_COLOR;
use druid::widget::*;
use druid::{Data, Insets, TextAlignment, Widget};

pub fn button_builder<T: Data>(text: &str) -> impl Widget<T> {
    Label::new(text)
        .with_text_alignment(TextAlignment::Center)
        .center()
        .padding(4.0)
        .background(CARD_COLOR)
        .rounded(2.0)
        .padding(Insets::uniform_xy(0., 2.))
        .expand_width()
        .controller(ClickableArea)
}
