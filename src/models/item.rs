use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddItemResponse {
    pub success: bool,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
}
