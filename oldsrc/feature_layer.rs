use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::parser::parse_response;

#[derive(Debug, Clone)]
pub struct FeatureLayer {
    //pub name: String,
    pub url: String,
    pub metadata: MetaData,
    client: Client,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaData {
    pub r#type: String, // should be Feature Layer
    pub name: String,   // name of the layer
    pub fields: Vec<EsriField>,
    //max_record_count: i32, // TODO: use this to dynamically handle page size
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EsriField {
    pub name: String,
    pub alias: String,
    pub r#type: EsriType,
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

// pub struct Feature {
//     pub geometry: serde_json::Value,
//     pub attributes: serde_json::Value,
// }

#[derive(Default, Serialize)]
pub struct UpdateFeaturesRequest {
    pub features: Vec<serde_json::Value>,
    #[serde(rename = "gdbVersion")]
    pub gdb_version: Option<String>,
    #[serde(rename = "returnEditMoment")]
    pub return_edit_moment: Option<bool>,
    #[serde(rename = "rollbackOnFailure")]
    pub rollback_on_failure: Option<bool>,
    #[serde(rename = "trueCurveClient")]
    pub true_curve_client: Option<bool>,
    #[serde(rename = "timeReferenceUnknownClient")]
    pub time_reference_unknown_client: Option<String>,
    #[serde(rename = "useUniqueIds")]
    pub use_unique_ids: Option<bool>,
    pub f: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateFeaturesResponse {
    #[serde(rename = "updateResults")]
    pub update_results: Vec<UpdateResult>,
}

#[derive(Deserialize)]
pub struct UpdateResult {
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[serde(rename = "globalId")]
    pub global_id: String,
    pub success: bool,
    pub error: Option<UpdateResultError>,
}

#[derive(Deserialize)]
pub struct UpdateResultError {
    pub code: i32,
    pub message: String,
}

impl FeatureLayer {
    pub async fn new(client: &Client, url: &str) -> anyhow::Result<FeatureLayer> {
        let response = client.get(url).query(&[("f", "json")]).send().await?;

        let metadata = parse_response::<MetaData>(response)
            .await
            .expect("Failed to fetch feature service metadata");

        Ok(FeatureLayer {
            url: url.to_string(),
            metadata,
            client: client.clone(),
        })
    }

    pub async fn update_features(
        &self,
        features: Vec<serde_json::Value>,
    ) -> anyhow::Result<UpdateFeaturesResponse> {
        let url = format!("{}/updateFeatures", self.url);
        let request = UpdateFeaturesRequest {
            features,
            ..Default::default()
        };
        let response = self.client.post(&url).json(&request).send().await?;
        let result = parse_response::<UpdateFeaturesResponse>(response).await?;
        Ok(result)
    }
}
