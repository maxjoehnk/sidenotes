use super::FlagField;
use druid::{Widget, WidgetExt};
use std::marker::PhantomData;

pub struct ProviderSettingsFlagRow<T, L> {
    title: &'static str,
    lens: L,
    _data: PhantomData<T>,
}

impl<T: druid::Data, L: druid::Lens<T, bool> + Clone> ProviderSettingsFlagRow<T, L> {
    pub fn new(title: &'static str, lens: L) -> Self {
        Self {
            title,
            lens,
            _data: Default::default(),
        }
    }

    pub(super) fn build_view(&self) -> impl Widget<T> {
        let field = FlagField::view(self.title);
        field.lens(self.lens.clone())
    }

    pub(super) fn build_edit(&self) -> impl Widget<T> {
        let field = FlagField::edit(self.title);
        field.lens(self.lens.clone())
    }
}
