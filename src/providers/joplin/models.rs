use crate::rich_text::Markdown;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct TodoNote {
    pub id: String,
    pub title: String,
    pub body: Markdown,
    pub notebook: Notebook,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Notebook {
    pub id: String,
    pub title: String,
    pub parent_id: Option<String>,
}

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

#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
    pub id: String,
    pub parent_id: Option<String>,
    pub title: String,
}
