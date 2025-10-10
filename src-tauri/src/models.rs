use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub uuid: Uuid,
    pub username: String,
    pub avatar_url: String,
    pub account_type: AccountType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    Microsoft,
    Offline,
}