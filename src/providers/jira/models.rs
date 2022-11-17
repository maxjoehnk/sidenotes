use crate::rich_text::JiraMarkup;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct SearchResponse {
    pub issues: Vec<Issue>,
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
    pub components: Vec<Component>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Status {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueWithComments {
    pub id: String,
    pub key: String,
    pub fields: IssueCommentsFields,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueCommentsFields {
    pub comment: CommentField,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommentField {
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Comment {
    pub id: String,
    pub author: CommentAuthor,
    pub body: JiraMarkup,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentAuthor {
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
}
