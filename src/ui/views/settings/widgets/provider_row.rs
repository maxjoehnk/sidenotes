use super::SettingsField;
use druid::{Widget, WidgetExt};
use std::marker::PhantomData;

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

impl<T: druid::Data, L> ProviderSettingsRow<T, L> {
    pub fn secret(mut self) -> Self {
        self.secret = true;
        self
    }

    pub fn multiline(mut self) -> Self {
        self.multiline = true;
        self
    }
}
