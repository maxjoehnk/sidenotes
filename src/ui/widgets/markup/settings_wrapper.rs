use crate::rich_text::RawRichText;
use crate::DISABLE_COLORIZED_BACKGROUNDS;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget, WidgetPod,
};

pub struct MarkupSettingsWrapper<W> {
    inner: WidgetPod<(RawRichText, bool), W>,
}

impl<W: Widget<(RawRichText, bool)>> MarkupSettingsWrapper<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner: WidgetPod::new(inner),
        }
    }

    fn get_inner_data(data: &RawRichText, env: &Env) -> (RawRichText, bool) {
        (data.clone(), env.get(DISABLE_COLORIZED_BACKGROUNDS))
    }
}

impl<W: Widget<(RawRichText, bool)>> Widget<RawRichText> for MarkupSettingsWrapper<W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut RawRichText, env: &Env) {
        let mut inner_data = Self::get_inner_data(data, env);
        self.inner.event(ctx, event, &mut inner_data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &RawRichText,
        env: &Env,
    ) {
        let inner_data = Self::get_inner_data(data, env);
        self.inner.lifecycle(ctx, event, &inner_data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &RawRichText, data: &RawRichText, env: &Env) {
        let inner_data = Self::get_inner_data(data, env);
        self.inner.update(ctx, &inner_data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &RawRichText,
        env: &Env,
    ) -> Size {
        let inner_data = Self::get_inner_data(data, env);
        self.inner.layout(ctx, bc, &inner_data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &RawRichText, env: &Env) {
        let inner_data = Self::get_inner_data(data, env);
        self.inner.paint(ctx, &inner_data, env);
    }
}
