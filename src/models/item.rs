use serde::{Deserialize, Serialize};

use crate::models::{BaseMap, InitialState, OperationalLayer, SpatialReference};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddItemResponse {
    pub success: bool,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublishItemResponse {
    pub services: Vec<PublishItemService>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublishItemService {
    #[serde(rename = "encodedServiceURL")]
    pub encoded_service_url: String,
    pub job_id: String,
    pub service_item_id: String,
    pub serviceurl: String,
    #[serde(rename = "type")]
    pub service_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateItemResponse {
    pub success: bool,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteItemResponse {
    pub success: bool,
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddResourcesResponse {
    pub success: bool,
    pub item_id: String,
    pub owner: String,
    pub folder: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeResponse {
    pub publish_parameters: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebMapDataJson {
    pub authoring_app: String,
    pub authoring_app_version: String,
    pub base_map: BaseMap,
    pub initial_state: Option<InitialState>,
    pub operational_layers: Vec<OperationalLayer>,
    pub spatial_reference: SpatialReference,
    pub time_zone: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourcesResponse {
    pub total: i64,
    pub start: i64,
    pub num: i64,
    pub next_start: i64,
    pub resources: Vec<Resource>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub resource: String,
    pub created: i64,
    pub size: i64,
}
