use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::group::Group;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserSelfResponse {
    pub username: String,
    pub full_name: String,
    pub preferred_view: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub access: String,
    pub storage_usage: Option<i64>,
    pub storage_quota: Option<i64>,
    pub org_id: Option<String>,
    pub role: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub culture: Option<String>,
    pub region: Option<String>,
    pub thumbnail: Option<String>,
    pub created: Option<i64>,
    pub modified: Option<i64>,
    #[serde(default)]
    pub groups: Vec<Group>,
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}
