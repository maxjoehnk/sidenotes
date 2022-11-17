use crate::providers::ProviderSettings;
use crate::ui::views::settings::widgets::{SettingsBuilder, SettingsRow};
use druid::text::{Selection, Validation, ValidationError};
use druid::widget::*;
use druid::{Color, Data, FontDescriptor, FontFamily, FontWeight, Widget};

use crate::ui::widgets::header_builder;

pub trait ProviderSettingsBuilder<T: druid::Data> {
    type ViewWidget: Widget<(T, ProviderSettings)>;
    type EditWidget: Widget<(T, ProviderSettings)>;

    fn build_view(self) -> Self::ViewWidget;
    fn build_edit(self) -> Self::EditWidget;
}

impl<T: druid::Data> ProviderSettingsBuilder<T> for SettingsBuilder<T> {
    type ViewWidget = Flex<(T, ProviderSettings)>;
    type EditWidget = Flex<(T, ProviderSettings)>;

    fn build_view(self) -> Self::ViewWidget {
        let mut column = Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Fill)
            .with_child(settings_title(self.title).lens(druid::lens!((T, ProviderSettings), 1)))
            .with_child(settings_subtitle(self.title))
            .with_spacer(8.);
        for (_, view) in self.fields {
            column.add_child(view.lens(druid::lens!((T, ProviderSettings), 0)));
        }

        column
    }

    fn build_edit(self) -> Self::EditWidget {
        let mut column = Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Fill)
            .with_child(settings_header(self.title).lens(druid::lens!((T, ProviderSettings), 1)));
        column.add_child(
            provider_name_builder(self.title).lens(druid::lens!((T, ProviderSettings), 1)),
        );
        for (edit, _) in self.fields {
            column.add_child(edit.lens(druid::lens!((T, ProviderSettings), 0)));
        }

        column
    }
}

pub fn settings_title(text: &str) -> impl Widget<ProviderSettings> {
    let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(18.0)
        .with_weight(FontWeight::BOLD);

    let text = text.to_string();
    Label::dynamic(move |settings: &ProviderSettings, _| {
        settings.name.clone().unwrap_or_else(|| text.clone())
    })
    .with_font(font)
}

pub fn settings_subtitle<T: Data>(text: &str) -> impl Widget<T> {
    Label::new(text)
        .with_text_size(14.0)
        .with_text_color(Color::rgb(0.6, 0.6, 0.6))
}

pub fn settings_header<T: Data>(title: &str) -> impl Widget<T> + '_ {
    header_builder(title)
}

fn provider_name_builder(provider_name: &str) -> impl Widget<ProviderSettings> {
    let editor = TextBox::new()
        .with_placeholder(provider_name)
        .with_formatter(OptionalFormatter);

    SettingsRow::new("Name", editor).lens(ProviderSettings::name)
}

struct OptionalFormatter;

impl druid::text::Formatter<Option<String>> for OptionalFormatter {
    fn format(&self, value: &Option<String>) -> String {
        value.clone().unwrap_or_default()
    }

    fn validate_partial_input(&self, _: &str, _: &Selection) -> Validation {
        Validation::success()
    }

    fn value(&self, input: &str) -> Result<Option<String>, ValidationError> {
        if input.is_empty() {
            Ok(None)
        } else {
            Ok(Some(input.to_string()))
        }
    }
}
