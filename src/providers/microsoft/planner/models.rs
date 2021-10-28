use serde::Deserialize;
use graph_rs_sdk::error::ErrorMessage;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum GraphResponse<T> {
    Ok(T),
    Error(ErrorMessage)
}

impl<T> GraphResponse<T> {
    pub fn into_result(self) -> anyhow::Result<T> {
        match self {
            Self::Ok(value) => Ok(value),
            Self::Error(err) => Err(anyhow::anyhow!("{:?}", err))
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlannerListResponse<T> {
    pub value: Vec<T>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannerTask {
    pub id: String,
    pub plan_id: String,
    pub bucket_id: String,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannerTaskDetails {
    pub description: String,
    pub id: String,
    pub preview_type: String,
    pub checklist_items: Vec<PlannerChecklistItem>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannerChecklistItem {
    pub is_checked: bool,
    pub title: String,
}
