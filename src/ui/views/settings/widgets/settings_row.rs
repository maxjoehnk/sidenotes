use druid::widget::*;
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Size, UpdateCtx, Widget, WidgetPod,
};

pub struct SettingsRow<T: Data> {
    widget: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T: Data> SettingsRow<T> {
    pub fn new(name: &str, widget: impl Widget<T> + 'static) -> Self {
        let widget = Flex::row()
            .must_fill_main_axis(true)
            .with_flex_child(Label::new(name).expand_width(), 1.5)
            .with_spacer(8.0)
            .with_flex_child(
                widget.expand_width(),
                FlexParams::new(2.0, CrossAxisAlignment::End),
            )
            .padding(Insets::uniform_xy(4., 4.));

        Self {
            widget: WidgetPod::new(widget.boxed()),
        }
    }
}

impl<T: Data> Widget<T> for SettingsRow<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.widget.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.widget.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &T, data: &T, env: &Env) {
        self.widget.update(ctx, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.widget.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.widget.paint(ctx, data, env)
    }
}
