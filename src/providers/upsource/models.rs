use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewList {
    pub reviews: Vec<ReviewDescriptor>,
    pub has_more: bool,
    pub total_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewDescriptor {
    pub review_id: ReviewId,
    pub title: String,
    pub description: Option<String>,
    pub state: ReviewState,
    pub is_unread: Option<bool>,
    pub is_ready_to_close: Option<bool>,
    #[serde(default)]
    pub branch: Vec<String>,
    #[serde(default)]
    pub issue: Vec<IssueId>,
    pub is_removed: Option<bool>,
    pub created_at: i64,
    pub created_by: Option<String>,
    pub updated_at: i64,
    pub completion_rate: CompletionRate,
    pub discussion_counter: Option<SimpleDiscussionCounter>,
    pub deadline: Option<i64>,
    pub is_muted: Option<bool>,
    #[serde(default)]
    pub labels: Vec<Label>,
    pub merge_from_branch: Option<String>,
    pub merge_to_branch: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewId {
    pub project_id: String,
    pub review_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueId {
    pub issue_id: String,
    pub issue_link: Option<String>,
    pub is_created_from_upsource: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ReviewState {
    Open = 1,
    Closed = 2,
}

impl Display for ReviewState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewState::Open => write!(f, "Open"),
            ReviewState::Closed => write!(f, "Closed"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionRate {
    pub completed_count: i32,
    pub reviewers_count: i32,
    pub has_concern: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleDiscussionCounter {
    pub count: i32,
    pub has_unresolved: bool,
    pub unresolved_count: i32,
    pub resolved_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: Option<String>,
    pub name: String,
    pub color_id: Option<String>,
}
