use crate::CARD_COLOR;
use druid::widget::*;
use druid::{Command, Env, Event, EventCtx, Insets, KbKey, Target, TextAlignment, Widget};
use druid_widget_nursery::prism;

use crate::models::*;
use crate::ui::commands;
use crate::ui::commands::PROVIDER_ACTION;
use crate::ui::lens::Link;
use crate::ui::prism::{TodoBody, TodoLink};
use crate::ui::widgets::{header_builder, ClickableArea};

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
                ctx.submit_command(Command::new(commands::CLOSE_TODO, (), Target::Auto));
                return;
            }
        }
        child.event(ctx, event, data, env)
    }
}

pub fn detail_builder() -> impl Widget<Todo> {
    let header = header_builder(
        |item: &Todo, _env: &_| item.title.clone(),
        Command::new(commands::CLOSE_TODO, (), Target::Auto),
    );
    let body = RawLabel::new().with_line_break_mode(LineBreaking::WordWrap);
    let body = prism::PrismWrap::new(body, TodoBody);
    let body = Scroll::new(body).vertical().expand_height();
    let link = prism::PrismWrap::new(link_builder(), TodoLink);
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
