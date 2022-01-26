use druid::widget::*;
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, FontWeight, Insets,
    LayoutCtx, Lens, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetPod,
};
use std::marker::PhantomData;

use crate::ui::widgets::header_builder;

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
            if self.password {
                Label::new("******").boxed()
            } else {
                let text_box = if self.multiline {
                    TextBox::multiline()
                } else {
                    TextBox::new()
                };
                text_box.with_placeholder(self.text).boxed()
            }
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

pub fn settings_title<T: Data>(text: &str) -> impl Widget<T> {
    let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(18.0)
        .with_weight(FontWeight::BOLD);

    Label::new(text).with_font(font)
}

pub fn settings_header<T: Data>(title: &str) -> impl Widget<T> + '_ {
    header_builder(title)
}

type BuiltSettingsRow<T> = (Box<dyn Widget<T>>, Box<dyn Widget<T>>);

pub struct SettingsBuilder<T: druid::Data> {
    title: &'static str,
    fields: Vec<BuiltSettingsRow<T>>,
}

impl<'a, T: druid::Data> SettingsBuilder<T> {
    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            fields: Vec::new(),
        }
    }

    pub fn add_field<L: Lens<T, String> + Clone + 'static>(
        mut self,
        row: ProviderSettingsRow<T, L>,
    ) -> Self {
        let edit = row.build_edit();
        let edit = edit.boxed();
        let view = row.build_view();
        let view = view.boxed();
        self.fields.push((edit, view));
        self
    }

    pub(super) fn build_view(self) -> impl Widget<T> {
        let mut column = Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Fill)
            .with_child(settings_title(self.title));
        for (_, view) in self.fields {
            column.add_child(view);
        }

        column
    }

    pub(super) fn build_edit(self) -> impl Widget<T> {
        let mut column = Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Fill)
            .with_child(settings_header(self.title));
        for (edit, _) in self.fields {
            column.add_child(edit);
        }

        column
    }
}

pub struct ProviderSettingsRow<T, L> {
    title: &'static str,
    secret: bool,
    multiline: bool,
    lens: L,
    _data: PhantomData<T>,
}

impl<T: druid::Data, L: druid::Lens<T, String> + Clone> ProviderSettingsRow<T, L> {
    pub fn new(title: &'static str, lens: L) -> Self {
        Self {
            title,
            secret: false,
            multiline: false,
            lens,
            _data: Default::default(),
        }
    }

    pub fn secret(mut self) -> Self {
        self.secret = true;
        self
    }

    pub fn multiline(mut self) -> Self {
        self.multiline = true;
        self
    }

    pub(super) fn build_view(&self) -> impl Widget<T> {
        let mut field = SettingsField::view(self.title);
        if self.multiline {
            field = field.with_multiline();
        }
        if self.secret {
            field = field.with_password();
        }
        field.lens(self.lens.clone())
    }

    pub(super) fn build_edit(&self) -> impl Widget<T> {
        let mut field = SettingsField::edit(self.title);
        if self.multiline {
            field = field.with_multiline();
        }
        if self.secret {
            field = field.with_password();
        }
        field.lens(self.lens.clone())
    }
}
