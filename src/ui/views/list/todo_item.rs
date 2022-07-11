use chrono::Datelike;
use druid::widget::*;
use druid::{Color, Command, FontDescriptor, FontFamily, Insets, Target, Widget};
use druid_widget_nursery::prism;

use crate::models::*;
use crate::ui::commands;
use crate::ui::lazy_icon::{IconExtensions, IconLoader, LazyIcon};
use crate::ui::prism::TodoDueDate;
use crate::ui::theme::{CARD_COLOR, STATUS_COLOR};
use crate::ui::widgets::ClickableArea;

thread_local! {
    static DUE_DATE_ICON: LazyIcon = LazyIcon::new(|| {
        include_str!("../../../../assets/icons/due_date.svg").load()
    });
}

pub fn todo_builder() -> impl Widget<Todo> {
    let state = Either::new(
        |todo: &Todo, _: &_| todo.state.is_some(),
        todo_item_builder(tags_with_state_builder()),
        todo_item_builder(tags_builder()),
    );

    state
        .padding(4.0)
        .background(CARD_COLOR)
        .rounded(2.0)
        .padding(Insets::uniform_xy(0., 2.))
        .expand_width()
        .controller(ClickableArea)
        .on_click(|event_ctx, todo: &mut Todo, _: &_| {
            event_ctx.submit_command(Command::new(
                commands::NAVIGATE,
                Navigation::Selected(todo.clone()),
                Target::Auto,
            ))
        })
}

fn tag_builder() -> impl Widget<String> {
    Label::new(|tag: &String, _env: &_| tag.clone())
        .with_text_color(Color::BLACK)
        .padding(2.0)
        .background(STATUS_COLOR)
        .rounded(2.0)
}

fn todo_title_builder() -> impl Widget<Todo> {
    Label::new(|item: &Todo, _env: &_| item.title.clone())
        .with_line_break_mode(LineBreaking::WordWrap)
}

fn tags_builder() -> impl Widget<Todo> {
    List::new(tag_builder).with_spacing(4.).lens(Todo::tags)
}

fn tags_with_state_builder() -> impl Widget<Todo> {
    let state = Label::new(|todo: &Todo, _env: &_| todo.state.clone().unwrap_or_default())
        .with_text_color(Color::BLACK)
        .padding(2.0)
        .background(STATUS_COLOR)
        .rounded(2.0);

    Flex::row()
        .with_child(state)
        .with_spacer(4.)
        .with_child(tags_builder())
}

fn todo_item_builder(tag_row: impl Widget<Todo> + 'static) -> impl Widget<Todo> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(todo_title_builder())
        .with_spacer(4.0)
        .with_child(tag_row)
        .with_child(due_builder())
}

fn due_builder() -> impl Widget<Todo> {
    prism::PrismWrap::new(due_date_builder(), TodoDueDate)
}

fn due_date_builder() -> impl Widget<LocalDateTime> {
    let time_font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.0);
    let icon = DUE_DATE_ICON.to_svg().fix_size(16., 16.).padding(8.);

    let date = Label::new(|due_date: &LocalDateTime, _: &_| {
        if due_date.is_today() {
            "Today".to_string()
        } else {
            format!(
                "{}.{}.{}",
                due_date.day(),
                due_date.month(),
                due_date.year()
            )
        }
    })
    .with_font(time_font)
    .with_text_color(druid::theme::PLACEHOLDER_COLOR)
    .align_left();

    Flex::row().with_child(icon).with_child(date)
}
