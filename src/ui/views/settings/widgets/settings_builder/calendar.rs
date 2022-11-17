use crate::ui::views::settings::widgets::SettingsBuilder;
use druid::widget::*;
use druid::{Data, FontDescriptor, FontFamily, FontWeight, Widget};

use crate::ui::widgets::header_builder;

pub trait CalendarSettingsBuilder<T: Data> {
    type ViewWidget: Widget<T>;
    type EditWidget: Widget<T>;

    fn build_view(self) -> Self::ViewWidget;
    fn build_edit(self) -> Self::EditWidget;
}

impl<T: druid::Data> CalendarSettingsBuilder<T> for SettingsBuilder<T> {
    type ViewWidget = Flex<T>;
    type EditWidget = Flex<T>;

    fn build_view(self) -> Self::ViewWidget {
        let mut column = Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Fill)
            .with_child(settings_title(self.title));
        for (_, view) in self.fields {
            column.add_child(view);
        }

        column
    }

    fn build_edit(self) -> Self::EditWidget {
        let mut column = Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Fill)
            .with_child(header_builder(self.title));
        for (edit, _) in self.fields {
            column.add_child(edit);
        }

        column
    }
}

pub fn settings_title<T: Data>(text: &str) -> impl Widget<T> {
    let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(18.0)
        .with_weight(FontWeight::BOLD);

    Label::new(text).with_font(font)
}
