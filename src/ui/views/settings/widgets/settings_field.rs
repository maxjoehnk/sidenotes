use super::settings_row::SettingsRow;
use druid::widget::*;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget, WidgetPod,
};

pub struct SettingsField<'a> {
    config: SettingsFieldConfig<'a>,
    widget: WidgetPod<String, Box<dyn Widget<String>>>,
}

struct SettingsFieldConfig<'a> {
    text: &'a str,
    edit: bool,
    password: bool,
    multiline: bool,
}

impl<'a> SettingsFieldConfig<'a> {
    fn view(text: &'a str) -> Self {
        Self {
            text,
            edit: false,
            password: false,
            multiline: false,
        }
    }

    fn edit(text: &'a str) -> Self {
        Self {
            edit: true,
            ..Self::view(text)
        }
    }

    fn build(&self) -> impl Widget<String> {
        let widget = if self.edit {
            let text_box = if self.multiline {
                TextBox::multiline()
            } else {
                TextBox::new()
            };
            text_box.with_placeholder(self.text).boxed()
        } else {
            self.build_view().boxed()
        };

        SettingsRow::new(self.text, widget)
    }

    fn build_view(&self) -> impl Widget<String> {
        let mut label = if self.password {
            Label::new("******")
        } else {
            Label::new(move |value: &String, _: &_| value.clone())
        };
        if self.multiline {
            label.set_line_break_mode(LineBreaking::WordWrap);
        }
        label
    }
}

impl<'a> SettingsField<'a> {
    pub fn view(text: &'a str) -> Self {
        let config = SettingsFieldConfig::view(text);
        let widget = config.build();
        Self {
            config,
            widget: WidgetPod::new(widget.boxed()),
        }
    }

    pub fn edit(text: &'a str) -> Self {
        let config = SettingsFieldConfig::edit(text);
        let widget = config.build();
        Self {
            config,
            widget: WidgetPod::new(widget.boxed()),
        }
    }

    pub fn with_multiline(mut self) -> Self {
        self.config.multiline = true;
        self.widget = WidgetPod::new(self.config.build().boxed());
        self
    }

    pub fn with_password(mut self) -> Self {
        self.config.password = true;
        self.widget = WidgetPod::new(self.config.build().boxed());
        self
    }
}

impl<'a> Widget<String> for SettingsField<'a> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
        self.widget.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &String, env: &Env) {
        self.widget.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &String, data: &String, env: &Env) {
        self.widget.update(ctx, data, env)
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &String,
        env: &Env,
    ) -> Size {
        self.widget.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
        self.widget.paint(ctx, data, env)
    }
}
