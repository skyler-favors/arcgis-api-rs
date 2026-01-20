use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PortalsSelfResponse {
    pub id: String,
    // Future fields can be added here as needed
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}
