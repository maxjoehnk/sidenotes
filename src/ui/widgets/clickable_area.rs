use druid::widget::*;
use druid::{Cursor, Data, Env, Event, EventCtx, Widget};

pub struct ClickableArea;

impl<T: Data, W: Widget<T>> Controller<T, W> for ClickableArea {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if ctx.is_hot() {
            ctx.set_cursor(&Cursor::Pointer)
        } else {
            ctx.set_cursor(&Cursor::Arrow)
        }
        child.event(ctx, event, data, env)
    }
}
