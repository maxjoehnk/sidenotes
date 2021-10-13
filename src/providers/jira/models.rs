use serde::Deserialize;

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
    pub description: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Status {
    pub name: String
}
