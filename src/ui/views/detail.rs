use crate::CARD_COLOR;
use druid::widget::*;
use druid::{
    Command, Env, Event, EventCtx, FontDescriptor, FontFamily, FontWeight, Insets, KbKey, Target,
    TextAlignment, Widget,
};

use crate::models::*;
use crate::ui::commands;
use crate::ui::commands::PROVIDER_ACTION;
use crate::ui::lens::Link;
use crate::ui::widgets::{header_builder, markup_builder, ClickableArea};

struct DetailController;

impl<W: Widget<Todo>> Controller<Todo, W> for DetailController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Todo,
        env: &Env,
    ) {
        if let Event::KeyDown(event) = event {
            println!("{:?}", event.key);
            if event.key == KbKey::BrowserBack {
                ctx.submit_command(Command::new(commands::NAVIGATE_BACK, (), Target::Auto));
                return;
            }
        }
        child.event(ctx, event, data, env)
    }
}

pub fn detail_builder() -> impl Widget<Todo> {
    let header = header_builder(|item: &Todo, _env: &_| item.title.clone());
    let link = Maybe::or_empty(link_builder).lens(Todo::link);
    let body = Maybe::or_empty(markup_builder).lens(Todo::body);
    let body = Flex::column()
        .with_child(body)
        .with_child(comments_builder());
    let body = Scroll::new(body).vertical().expand_height();
    let actions = List::new(action_builder).lens(Todo::actions);

    Flex::column()
        .with_child(header)
        .with_child(link)
        .with_flex_child(body, 1.)
        .with_child(actions)
        .must_fill_main_axis(true)
        .controller(DetailController)
}

fn link_builder() -> impl Widget<String> {
    RawLabel::new()
        .with_line_break_mode(LineBreaking::WordWrap)
        .lens(Link)
}

fn action_builder() -> impl Widget<TodoAction> {
    Label::dynamic(|action: &TodoAction, _| action.label.to_string())
        .with_text_alignment(TextAlignment::Center)
        .center()
        .padding(4.0)
        .background(CARD_COLOR)
        .rounded(2.0)
        .padding(Insets::uniform_xy(0., 2.))
        .expand_width()
        .controller(ClickableArea)
        .on_click(|ctx, action: &mut TodoAction, _| {
            ctx.submit_command(Command::new(PROVIDER_ACTION, *action, Target::Auto))
        })
}

fn comments_builder() -> impl Widget<Todo> {
    let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(16.0)
        .with_weight(FontWeight::BOLD);
    let label = Label::dynamic(|todo: &Todo, _| format!("Comments ({})", todo.comments.len()))
        .with_font(font);
    let comments = Flex::column()
        .with_spacer(16.0)
        .with_child(label)
        .with_child(List::new(comment_builder).lens(Todo::comments));

    Either::new(|todo, _| todo.comments.is_empty(), Flex::row(), comments)
}

fn comment_builder() -> impl Widget<TodoComment> {
    let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_weight(FontWeight::BOLD);
    let author = Label::new(|item: &TodoComment, _env: &_| item.author.clone().unwrap_or_default())
        .with_font(font)
        .with_line_break_mode(LineBreaking::WordWrap);
    let body = markup_builder().lens(TodoComment::text);

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(author)
        .with_spacer(4.0)
        .with_child(body)
        .padding(4.0)
        .background(CARD_COLOR)
        .rounded(2.0)
        .padding(Insets::new(0., 2., 8., 2.))
        .expand_width()
}
