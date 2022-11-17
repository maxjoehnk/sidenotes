use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: Status,
    pub item: TaskItem,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Done,
    Todo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskItem {
    pub title: String,
    pub url: String,
}
