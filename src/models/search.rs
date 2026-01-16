use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub start: i64,
    #[serde(default)]
    pub num: i64,
    #[serde(default = "default_next_start")]
    pub next_start: i64,
    #[serde(default)]
    pub results: Vec<Item>,
}

fn default_next_start() -> i64 {
    -1
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub owner: String,
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub modified: i64,
    pub guid: Option<String>,
    pub name: Option<String>,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub type_keywords: Vec<String>,
    pub description: Option<String>,
    pub snippet: Option<String>,
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub extent: Vec<Vec<f64>>,
    pub spatial_reference: Option<String>,
    #[serde(default)]
    pub access_information: Value,
    pub license_info: Option<String>,
    pub culture: Option<String>,
    #[serde(default)]
    pub url: Value,
    #[serde(default)]
    pub proxy_filter: Value,
    pub access: Option<String>,
    #[serde(default)]
    pub size: i64,
    #[serde(default)]
    pub properties: Value,
    #[serde(default)]
    pub app_categories: Vec<Value>,
    #[serde(default)]
    pub industries: Vec<Value>,
    #[serde(default)]
    pub languages: Vec<Value>,
    #[serde(default)]
    pub large_thumbnail: Value,
    #[serde(default)]
    pub banner: Value,
    #[serde(default)]
    pub screenshots: Vec<Value>,
    #[serde(default)]
    pub listed: bool,
    pub owner_folder: Option<String>,
    #[serde(default)]
    pub protected: bool,
    #[serde(default)]
    pub num_comments: i64,
    #[serde(default)]
    pub num_ratings: i64,
    #[serde(default)]
    pub avg_rating: f64,
    #[serde(default)]
    pub num_views: i64,
    #[serde(default)]
    pub last_viewed: i64,
}
