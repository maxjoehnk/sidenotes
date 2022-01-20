use druid::widget::*;
use druid::{Command, Env, Event, EventCtx, KbKey, Target, Widget};
use druid_widget_nursery::prism;

use crate::models::*;
use crate::ui::commands;
use crate::ui::lens::Link;
use crate::ui::prism::{TodoBody, TodoLink};
use crate::ui::widgets::header_builder;

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
    let body = Scroll::new(body).vertical();
    let link = prism::PrismWrap::new(link_builder(), TodoLink);

    Flex::column()
        .with_child(header)
        .with_child(link)
        .with_flex_child(body, 1.)
        .controller(DetailController)
}

fn link_builder() -> impl Widget<String> {
    RawLabel::new()
        .with_line_break_mode(LineBreaking::WordWrap)
        .lens(Link)
}
