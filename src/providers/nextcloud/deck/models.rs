use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BoardModel {
    pub id: u32,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StackModel {
    pub id: u32,
    pub board_id: u32,
    pub title: String,
    #[serde(default)]
    pub cards: Vec<CardModel>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardModel {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub assigned_users: Vec<AssignedUserModel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssignedUserModel {
    pub id: u32,
    pub participant: DeckParticipant,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckParticipant {
    pub primary_key: String,
    pub uid: String,
    pub displayname: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OcsResponse<T> {
    pub ocs: OcsModel<T>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OcsModel<T> {
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatusModel {
    pub user_id: String,
}
