use crate::rich_text::Markdown;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Note {
    pub id: String,
    #[serde(rename = "parent_id")]
    pub notebook_id: String,
    pub title: String,
    pub body: Markdown,
    pub is_todo: u32,
    pub todo_due: u64,
    pub todo_completed: u64,
}

impl Note {
    pub(super) fn is_todo(&self) -> bool {
        self.is_todo == 1
    }

    pub(super) fn is_completed(&self) -> bool {
        self.todo_completed != 0
    }
}

#[derive(Debug, Deserialize)]
pub struct JoplinResponse<T> {
    pub items: Vec<T>,
    pub has_more: bool,
}
