use crate::ui::views::settings::widgets::SettingsRow;
use druid::widget::*;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget, WidgetPod,
};

pub struct FlagField {
    widget: WidgetPod<bool, Box<dyn Widget<bool>>>,
}

struct FlagFieldConfig<'a> {
    text: &'a str,
    edit: bool,
}

impl<'a> FlagFieldConfig<'a> {
    fn view(text: &'a str) -> Self {
        Self { text, edit: false }
    }

    fn edit(text: &'a str) -> Self {
        Self {
            edit: true,
            ..Self::view(text)
        }
    }

    fn build(&self) -> impl Widget<bool> {
        let widget = if self.edit {
            Switch::new().boxed()
        } else {
            Switch::new().disabled_if(|_, _| true).boxed()
        };

        SettingsRow::new(self.text, widget)
    }
}

impl FlagField {
    pub fn view(text: &str) -> Self {
        let config = FlagFieldConfig::view(text);
        let widget = config.build();

        Self {
            widget: WidgetPod::new(widget.boxed()),
        }
    }

    pub fn edit(text: &str) -> Self {
        let config = FlagFieldConfig::edit(text);
        let widget = config.build();

        Self {
            widget: WidgetPod::new(widget.boxed()),
        }
    }
}

impl Widget<bool> for FlagField {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut bool, env: &Env) {
        self.widget.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &bool, env: &Env) {
        self.widget.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &bool, data: &bool, env: &Env) {
        self.widget.update(ctx, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &bool, env: &Env) -> Size {
        self.widget.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &bool, env: &Env) {
        self.widget.paint(ctx, data, env)
    }
}
