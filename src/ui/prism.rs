use druid::text::RichText;
use druid_widget_nursery::prism::Prism;

use crate::models::*;
use crate::rich_text::IntoRichText;

pub struct TodoLink;

impl Prism<Todo, String> for TodoLink {
    fn get(&self, data: &Todo) -> Option<String> {
        data.link.clone()
    }

    fn put(&self, data: &mut Todo, inner: String) {
        data.link = Some(inner);
    }
}

pub struct TodoBody;

impl Prism<Todo, RichText> for TodoBody {
    fn get(&self, data: &Todo) -> Option<RichText> {
        data.body.clone().map(|body| body.into_rich_text())
    }

    fn put(&self, _: &mut Todo, _: RichText) {
        // Formatted body is readonly
    }
}

pub struct NavigationListPrism;
impl Prism<AppState, AppState> for NavigationListPrism {
    fn get(&self, data: &AppState) -> Option<AppState> {
        if matches!(data.navigation, Navigation::List) {
            Some(data.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: AppState) {
        if matches!(data.navigation, Navigation::List) {
            *data = inner;
        }
    }
}

pub struct NavigationSelectedPrism;
impl Prism<AppState, Todo> for NavigationSelectedPrism {
    fn get(&self, data: &AppState) -> Option<Todo> {
        if let Navigation::Selected(ref todo) = data.navigation {
            Some(todo.clone())
        } else {
            None
        }
    }

    fn put(&self, data: &mut AppState, inner: Todo) {
        if let Navigation::Selected(ref mut todo) = data.navigation {
            *todo = inner;
        }
    }
}
