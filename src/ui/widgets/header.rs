use crate::ui::commands;
use crate::ui::lazy_icon::*;
use crate::ui::widgets::ClickableArea;
use crate::CARD_COLOR;
use druid::widget::*;
use druid::{Command, Data, Target, Widget};

thread_local! {
    static BACK_BUTTON_ICON: LazyIcon = LazyIcon::new(|| {
        include_str!("../../../assets/icons/arrow-back.svg").load()
    });
}

pub fn header_builder<T: Data>(text: impl Into<LabelText<T>>) -> impl Widget<T> {
    let back_button = BACK_BUTTON_ICON
        .to_svg()
        .padding(8.)
        .controller(ClickableArea)
        .on_click(move |ctx: _, _: &mut T, _: &_| {
            ctx.submit_command(Command::new(commands::NAVIGATE_BACK, (), Target::Auto))
        });
    let title = Label::new(text)
        .with_line_break_mode(LineBreaking::WordWrap)
        .padding(4.);

    Flex::row()
        .with_child(back_button)
        .with_flex_child(title, 1.)
        .background(CARD_COLOR)
}
