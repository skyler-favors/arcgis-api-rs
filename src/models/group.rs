use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DeleteGroupsResponse {
    pub success: bool,
    #[serde(rename = "groupId")]
    pub group_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroupResponse {
    pub success: bool,
    pub group: Group,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub title: String,
    pub is_invitation_only: bool,
    pub is_view_only: bool,
    pub is_organization: Option<bool>,
    pub is_public: Option<bool>,
    pub is_read_only: bool,
    pub is_fav: bool,
    pub auto_join: bool,
    pub leaving_disallowed: bool,
    pub hidden_members: bool,
    pub membership_access: String,
    pub access: String,
    pub owner: String,
    pub description: Option<String>,
    pub snippet: Option<String>,
    pub phone: Option<String>,
    pub thumbnail: Option<String>,
    pub sort_field: Option<String>,
    pub sort_order: Option<String>,
    pub tags: Vec<String>,
    pub type_keywords: Vec<String>,
    pub capabilities: Vec<String>, // assuming strings in the array
    pub created: i64,
    pub modified: i64,
    pub notifications_enabled: bool,
    pub provider: Option<String>,
    pub protected: bool,
    pub provider_group_name: Option<String>,
    pub properties: Option<serde_json::Value>,
    pub featured_items_id: Option<String>,
    pub display_settings: Option<DisplaySettings>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplaySettings {
    pub item_types: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessLevel {
    Private,
    Org,
    Public,
}

impl Default for AccessLevel {
    fn default() -> Self {
        AccessLevel::Private
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SortField {
    Title,
    Owner,
    Avgrating,
    Numviews,
    Created,
    Modified,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Capability {
    UpdateItemControl,
    Distributed,
}

impl Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MembershipAccess {
    Org,
    Collaboration,
    None,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Contributors {
    Members,
    Owners,
}
