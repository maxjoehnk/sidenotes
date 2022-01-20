use std::time::Duration;

use druid::widget::prelude::*;
use druid::widget::Controller;
use druid::TimerToken;

static TIMER_INTERVAL: Duration = Duration::from_secs(10);

pub struct TimerController {
    timer_id: TimerToken,
}

impl Default for TimerController {
    fn default() -> Self {
        Self {
            timer_id: TimerToken::INVALID,
        }
    }
}

impl<T: Data, W: Widget<T>> Controller<T, W> for TimerController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        env: &Env,
    ) {
        if let Event::Timer(id) = event {
            if *id == self.timer_id {
                ctx.request_update();
                self.timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
        }
        child.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, child: &mut W, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.timer_id = ctx.request_timer(TIMER_INTERVAL);
        }
        child.lifecycle(ctx, event, data, env)
    }
}
