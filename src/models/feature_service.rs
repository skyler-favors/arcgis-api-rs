use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeatureServiceInfo {
    pub r#type: String, // should be Feature Layer
    pub name: String,   // name of the layer
    pub fields: Vec<EsriField>,
    //max_record_count: i32, // TODO: use this to dynamically handle page size
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EsriField {
    pub name: String,
    pub alias: String,
    pub r#type: EsriType,

    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
    // nullable: Option<bool>,
    // editable: bool,
    // default_value: Option<String>,
    // domain: Option<String>,
    // length: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EsriType {
    EsriFieldTypeOID,
    EsriFieldTypeGlobalID,
    EsriFieldTypeGUID,
    EsriFieldTypeString,
    EsriFieldTypeSmallInteger,
    EsriFieldTypeInteger,
    EsriFieldTypeDouble,
    EsriFieldTypeDate,
    EsriFieldTypeGeometry,
    EsriFieldTypeBigInteger,
    EsriFieldTypeSingle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeatureServiceQueryResponse {
    #[serde(default)]
    pub count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<EsriFeature>>,
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EsriFeature {
    pub attributes: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<serde_json::Value>,
}
