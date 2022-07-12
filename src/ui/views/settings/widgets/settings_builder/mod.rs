pub use self::calendar::*;
pub use self::provider::*;

use super::ProviderSettingsRow;
use druid::{Lens, Widget, WidgetExt};

mod calendar;
mod provider;

type BuiltSettingsRow<T> = (Box<dyn Widget<T>>, Box<dyn Widget<T>>);

pub struct SettingsBuilder<T: druid::Data> {
    pub(super) title: &'static str,
    pub(super) fields: Vec<BuiltSettingsRow<T>>,
}

impl<T: druid::Data> SettingsBuilder<T> {
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
}
