use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_urlencoded;
use std::collections::HashMap;

use crate::parser::parse_response;

pub struct PublishItemQuery {
    url: String,
    params: PublishItemQueryParams,
}

#[derive(Default)]
pub struct PublishItemQueryBuilder {
    url: String,
    params: PublishItemQueryParams,
}

/// `"type": "csv"`
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum PublishType {
    #[serde(rename = "csv")]
    #[default]
    Csv,
}

/// `locationType`: how to interpret the CSV (coords, address, lookup, none)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum LocationType {
    #[serde(rename = "coordinates")]
    Coordinates,
    #[serde(rename = "address")]
    Address,
    #[serde(rename = "lookup")]
    Lookup,
    #[serde(rename = "none")]
    #[default]
    None,
}

/// `coordinateFieldType`
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CoordinateFieldType {
    #[serde(rename = "Latitude And Longitude")]
    LatitudeAndLongitude,
    #[serde(rename = "MGRS")]
    Mgrs,
    #[serde(rename = "USNG")]
    Usng,
}

/// `candidateFieldsType`
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CandidateFieldsType {
    #[serde(rename = "Geometry Only")]
    GeometryOnly,
    #[serde(rename = "Minimal")]
    Minimal,
    #[serde(rename = "All Fields")]
    AllFields,
}

/// Generic spatial reference `{ "wkid": 4326, "latestWkid": 4326 }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialReference {
    pub wkid: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_wkid: Option<i32>,
}

/// `editorTrackingInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorTrackingInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_editor_tracking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ownership_access_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_others_to_query: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_others_to_update: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_others_to_delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_anonymous_to_update: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_anonymous_to_delete: Option<bool>,
}

/// `dateFieldsTimeReference`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateFieldsTimeReference {
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

/// CSV-specific `publishParameters` payload for /publish
///
/// This struct models only the CSV publish parameters JSON properties,
/// as described in the ArcGIS REST API docs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CsvPublishParameters {
    /// Must be `"csv"`.
    #[serde(rename = "type")]
    pub r#type: PublishType,

    /// (Required) Name of the service to be created.
    pub name: String,

    /// (Required) How to interpret the CSV: coordinates | address | lookup | none.
    pub location_type: LocationType,

    // --- Coordinate-based options (used when locationType == coordinates) ---
    /// Name of the field that contains the y-coordinate (lat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude_field_name: Option<String>,

    /// Name of the field that contains the x-coordinate (lon).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude_field_name: Option<String>,

    /// Type of coordinates (default: Latitude And Longitude).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_field_type: Option<CoordinateFieldType>,

    /// Name of the single field that stores coordinates, when using MGRS/USNG/etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_field_name: Option<String>,

    // --- Address / lookup options ---
    /// Used when locationType == address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_template: Option<String>,

    /// Used when locationType == lookup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_type: Option<String>,

    /// Used when locationType == lookup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_fields: Option<HashMap<String, String>>,

    /// Geocode service URL for batch geocoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geocode_service_url: Option<String>,

    /// Two-character country code (e.g. "us").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_country: Option<String>,

    /// Locale (e.g. "en").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_locale: Option<String>,

    /// Mapping from standardized address field roles to CSV field names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_fields: Option<HashMap<String, String>>,

    /// Mapping from standardized field names (Address, City, Region...) to locator fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standardized_field_names: Option<HashMap<String, String>>,

    // --- Layer definition ---
    /// (Required) Layer info descriptor (can be taken directly from Analyze).
    ///
    /// This is quite complex, so we treat it as arbitrary JSON and let
    /// the caller pass in the analyzer output or their own layer descriptor.
    pub layer_info: Value,

    // --- Optional metadata / behavior ---
    /// Description for the published dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Max record count for queries (default -1 means no constraint).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_record_count: Option<f64>,

    /// Copyright text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright_text: Option<String>,

    /// Column names (overrides names inferred from the CSV header).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_names: Option<Vec<String>>,

    /// Field delimiter (default `,`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_delimiter: Option<String>,

    /// Optional text qualifier (e.g. `"`), appears in example as `qualifier`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,

    /// Spatial reference of the input coordinates (default WKID 4326).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_sr: Option<SpatialReference>,

    /// Target spatial reference for storage (default WKID 102100 / 3857).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_sr: Option<SpatialReference>,

    /// Editor tracking configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_tracking_info: Option<EditorTrackingInfo>,

    /// What locator output fields to keep (`Geometry Only`, `Minimal`, `All Fields`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_fields_type: Option<CandidateFieldsType>,

    /// Time-zone info for date fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_fields_time_reference: Option<DateFieldsTimeReference>,

    // --- Extra CSV-related fields seen in the example JSON ---
    /// Optional source URL string (empty in the example).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,

    /// Hint string for source country (e.g. "").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_country_hint: Option<String>,

    /// Whether the resulting layer has static data (no edits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_static_data: Option<bool>,

    /// Whether to persist error records for review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist_error_records_for_review: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
}

impl CsvPublishParameters {
    pub fn to_urlencoded(&self) -> anyhow::Result<String> {
        Ok(serde_urlencoded::to_string(self)?)
    }

    pub fn json(&self, name: impl Into<String>, additional_fields: Vec<String>) -> String {
        let name = name.into();
        
        // Build fields array: always include Longitude and Latitude, then add additional fields
        let mut fields = serde_json::json!([
            {
                "name": "Longitude",
                "type": "esriFieldTypeDouble",
                "alias": "Longitude",
                "locationType": "longitude",
                "sqlType": "sqlTypeDouble"
            },
            {
                "name": "Latitude",
                "type": "esriFieldTypeDouble",
                "alias": "Latitude",
                "locationType": "latitude",
                "sqlType": "sqlTypeDouble"
            }
        ]);
        
        // Add additional fields as strings
        if let Some(fields_array) = fields.as_array_mut() {
            for field_name in &additional_fields {
                fields_array.push(serde_json::json!({
                    "name": field_name,
                    "type": "esriFieldTypeString",
                    "alias": field_name,
                    "sqlType": "sqlTypeOther",
                    "length": 256
                }));
            }
        }
        
        // Build template attributes: include all fields with null values
        let mut attributes = serde_json::json!({
            "Longitude": null,
            "Latitude": null
        });
        
        if let Some(attrs_obj) = attributes.as_object_mut() {
            for field_name in &additional_fields {
                attrs_obj.insert(field_name.clone(), serde_json::Value::Null);
            }
        }
        
        let json_obj = serde_json::json!({
            "type": "csv",
            "name": name,
            "sourceUrl": "",
            "maxRecordCount": 1000,
            "targetSR": {
                "wkid": 102100,
                "latestWkid": 3857
            },
            "editorTrackingInfo": {
                "enableEditorTracking": false,
                "enableOwnershipAccessControl": false,
                "allowOthersToUpdate": true,
                "allowOthersToDelete": false
            },
            "locationType": "coordinates",
            "latitudeFieldName": "Latitude",
            "longitudeFieldName": "Longitude",
            "sourceSR": {
                "wkid": 4326,
                "latestWkid": 4326
            },
            "columnDelimiter": ",",
            "layerInfo": {
                "currentVersion": 11.3,
                "id": 0,
                "name": "data",
                "type": "Feature Layer",
                "displayField": "",
                "description": "",
                "copyrightText": "",
                "defaultVisibility": true,
                "editFieldsInfo": null,
                "relationships": [],
                "isDataVersioned": false,
                "supportsRollbackOnFailureParameter": true,
                "supportsAdvancedQueries": true,
                "supportsValidateSQL": true,
                "supportsCalculate": true,
                "advancedQueryCapabilities": {
                    "supportsReturningQueryExtent": true,
                    "supportsStatistics": true,
                    "supportsDistinct": true,
                    "supportsPagination": true,
                    "supportsOrderBy": true,
                    "supportsQueryWithDistance": true
                },
                "geometryType": "esriGeometryPoint",
                "drawingInfo": {
                    "renderer": {
                        "type": "simple",
                        "symbol": {
                            "type": "esriSMS",
                            "style": "esriSMSCircle",
                            "color": [129, 140, 0, 255],
                            "size": 4,
                            "angle": 0,
                            "xoffset": 0,
                            "yoffset": 0,
                            "outline": {
                                "color": [0, 0, 0, 255],
                                "width": 1
                            }
                        },
                        "label": "",
                        "description": ""
                    }
                },
                "hasM": false,
                "hasZ": false,
                "allowGeometryUpdates": true,
                "hasAttachments": false,
                "htmlPopupType": "esriServerHTMLPopupTypeNone",
                "supportsApplyEditsWithGlobalIds": true,
                "objectIdField": "",
                "globalIdField": "",
                "typeIdField": "",
                "fields": fields,
                "types": [],
                "templates": [
                    {
                        "name": "New Feature",
                        "description": "",
                        "drawingTool": "esriFeatureEditToolPoint",
                        "prototype": {
                            "attributes": attributes
                        }
                    }
                ],
                "useStandardizedQueries": true,
                "enableZDefaults": false,
                "zDefault": 0,
                "supportedQueryFormats": "JSON",
                "hasStaticData": true,
                "maxRecordCount": 1000,
                "capabilities": "Query",
                "supportsCoordinatesQuantization": false,
                "supportsAttachmentsByUploadId": true
            },
            "coordinateFieldType": "LatitudeAndLongitude",
            "capabilities": "Query",
            "hasStaticData": true,
            "persistErrorRecordsForReview": true,
            "dateFieldsTimeReference": {
                "timeZone": "UTC"
            }
        });
        
        json_obj.to_string()
    }
}

// impl Default for CsvPublishParameters {
//     fn default() -> Self {
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishItemQueryParams {
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    pub file_type: String,
    pub publish_parameters: CsvPublishParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_initial_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
    pub f: String,
    pub token: Option<String>,
    #[serde(skip)]
    pub additional_fields: Vec<String>,
}

impl PublishItemQueryParams {
    pub fn to_urlencoded(&self) -> anyhow::Result<String> {
        let mut params = serde_json::to_value(self)?;
        // let publish_parameters_json = serde_json::to_string(&self.publish_parameters)?;
        // params["publishParameters"] = serde_json::json!(publish_parameters_json);
        params["publishParameters"] = serde_json::json!(self
            .publish_parameters
            .json(self.publish_parameters.name.clone(), self.additional_fields.clone()));
        Ok(serde_urlencoded::to_string(params)?)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublishItemResponse {
    pub services: Vec<PublishItemService>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublishItemService {
    #[serde(rename = "encodedServiceURL")]
    pub encoded_service_url: String,
    pub job_id: String,
    pub service_item_id: String,
    pub serviceurl: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl PublishItemQuery {
    pub fn builder(
        root: impl Into<String>,
        user_name: impl Into<String>,
        item_id: impl Into<String>,
    ) -> PublishItemQueryBuilder {
        PublishItemQueryBuilder::new(root, user_name, item_id)
    }

    pub async fn send(&self, client: &Client) -> anyhow::Result<PublishItemResponse> {
        let body = self
            .params
            .to_urlencoded()
            .expect("Failed to serialize publish item params");
        let response = client
            .post(&self.url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await?;
        let body = parse_response::<PublishItemResponse>(response).await?;
        Ok(body)
    }
}

impl PublishItemQueryBuilder {
    pub fn new(
        root: impl Into<String>,
        user_name: impl Into<String>,
        item_id: impl Into<String>,
    ) -> Self {
        // https://[root]/content/users/[userName]/publish

        let url = format!("{}/content/users/{}/publish", root.into(), user_name.into());
        // TODO: validtate url
        Self {
            url,
            params: PublishItemQueryParams {
                item_id: item_id.into(),
                f: "json".into(),
                file_type: "csv".into(),
                ..Default::default()
            },
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.publish_parameters.name = name.into();
        self
    }

    pub fn latitude_field_name(mut self, name: impl Into<String>) -> Self {
        self.params.publish_parameters.latitude_field_name = Some(name.into());
        self
    }

    pub fn longitude_field_name(mut self, name: impl Into<String>) -> Self {
        self.params.publish_parameters.longitude_field_name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.params.publish_parameters.description = Some(description.into());
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.params.token = Some(token.into());
        self
    }

    pub fn additional_fields(mut self, fields: Vec<String>) -> Self {
        self.params.additional_fields = fields;
        self
    }

    pub fn build(mut self) -> PublishItemQuery {
        let params = CsvPublishParameters {
            r#type: PublishType::Csv,
            name: self.params.publish_parameters.name,
            location_type: LocationType::Coordinates,

            latitude_field_name: self.params.publish_parameters.latitude_field_name.clone(),
            longitude_field_name: self.params.publish_parameters.longitude_field_name.clone(),
            coordinate_field_type: Some(CoordinateFieldType::LatitudeAndLongitude),

            // Required: layerInfo from /analyze
            layer_info: serde_json::json!({
                "fields": [
                    { "name": self.params.publish_parameters.longitude_field_name, "type": "esriFieldTypeDouble" },
                    { "name": self.params.publish_parameters.latitude_field_name,  "type": "esriFieldTypeDouble" }
                ],
                "geometryType": "esriGeometryPoint"
            }),

            description: self.params.publish_parameters.description,
            max_record_count: None,
            source_sr: Some(SpatialReference {
                wkid: 4326,
                latest_wkid: None,
            }),

            ..Default::default()
        };
        self.params.publish_parameters = params;

        let url = if let Some(token) = &self.params.token {
            format!("{}?token={}", self.url, token)
        } else {
            self.url
        };

        PublishItemQuery {
            url,
            params: self.params,
        }
    }
}
