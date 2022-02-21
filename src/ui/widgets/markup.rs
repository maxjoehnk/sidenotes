use crate::rich_text::{MarkupItem, RawRichText};
use crate::ui::lens::Markup;
use crate::ui::prism::MarkupTextPrism;
use crate::DISABLE_COLORIZED_BACKGROUNDS;
use druid::text::RichText;
use druid::widget::{Container, LineBreaking, List, RawLabel};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Size, UpdateCtx, Widget, WidgetExt, WidgetPod,
};
use druid_widget_nursery::enum_switcher::Switcher;

pub fn markup_builder() -> impl Widget<RawRichText> {
    let list = List::new(markup_item_builder).lens(Markup);

    MarkupSettingsWrapper::new(list)
}

fn markup_item_builder() -> impl Widget<MarkupItem> {
    let content = Switcher::new()
        .with_variant(MarkupTextPrism, markup_text_builder())
        .lens(MarkupItem::part);

    MarkupContainer::new(content).expand_width()
}

fn markup_text_builder() -> impl Widget<RichText> {
    RawLabel::new().with_line_break_mode(LineBreaking::WordWrap)
}

struct MarkupContainer {
    container: Container<MarkupItem>,
}

impl MarkupContainer {
    fn new(widget: impl Widget<MarkupItem> + 'static) -> Self {
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

struct MarkupSettingsWrapper<W> {
    inner: WidgetPod<(RawRichText, bool), W>,
}

impl<W: Widget<(RawRichText, bool)>> MarkupSettingsWrapper<W> {
    fn new(inner: W) -> Self {
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
