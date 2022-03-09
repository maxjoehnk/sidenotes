use crate::rich_text::MarkupItem;
use crate::DISABLE_COLORIZED_BACKGROUNDS;
use druid::widget::Container;
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Size, UpdateCtx, Widget,
};

pub struct MarkupContainer {
    container: Container<MarkupItem>,
}

impl MarkupContainer {
    pub fn new(widget: impl Widget<MarkupItem> + 'static) -> Self {
        Self {
            container: Container::new(widget),
        }
    }

    fn update_style(&mut self, data: &MarkupItem, env: &Env) {
        if env.get(DISABLE_COLORIZED_BACKGROUNDS) {
            self.container.clear_background();
            return;
        }
        if let Some(color) = data
            .style
            .background
            .as_ref()
            .and_then(|color| Color::from_hex_str(color).ok())
        {
            self.container.set_background(color);
        } else {
            self.container.clear_background();
        }
    }
}

impl Widget<MarkupItem> for MarkupContainer {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut MarkupItem, env: &Env) {
        self.container.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &MarkupItem,
        env: &Env,
    ) {
        self.container.lifecycle(ctx, event, data, env);
        if matches!(event, LifeCycle::WidgetAdded) {
            self.update_style(data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &MarkupItem, data: &MarkupItem, env: &Env) {
        self.container.update(ctx, old_data, data, env);
        self.update_style(data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &MarkupItem,
        env: &Env,
    ) -> Size {
        self.container.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &MarkupItem, env: &Env) {
        self.container.paint(ctx, data, env)
    }
}
