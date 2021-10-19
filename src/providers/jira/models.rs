use serde::Deserialize;
use crate::rich_text::JiraMarkup;

#[derive(Debug, Deserialize)]
pub(super) struct SearchResponse {
    pub issues: Vec<Issue>
}

#[derive(Debug, Clone, Deserialize)]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub fields: IssueFields,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueFields {
    pub summary: String,
    pub description: Option<JiraMarkup>,
    pub status: Status,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Status {
    pub name: String
}
