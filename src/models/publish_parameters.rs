use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSVPublishParameterBuilder {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,

    pub location_type: LocationType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude_field_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude_field_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_field_type: Option<CoordinateFieldType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_info: Option<LayerInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub max_record_count: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_names: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_delimiter: Option<String>,

    #[serde(rename = "sourceSR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_sr: Option<SourceSR>,

    #[serde(rename = "targetSR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_sr: Option<TargetSR>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_tracking_info: Option<EditorTrackingInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_fields_time_reference: Option<DateFieldsTimeReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_static_data: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist_error_records_for_review: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationType {
    #[default]
    Coordinates,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CoordinateFieldType {
    LatitudeAndLongitude,
    MGRS,
    USNG,
}

impl Default for CoordinateFieldType {
    fn default() -> Self {
        Self::LatitudeAndLongitude
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Capabilities {
    Query,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self::Query
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateFieldsTimeReference {
    pub time_zone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorTrackingInfo {
    pub allow_others_to_delete: bool,
    pub allow_others_to_update: bool,
    pub enable_editor_tracking: bool,
    pub enable_ownership_access_control: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerInfo {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub display_field: String,
    pub description: String,
    pub advanced_query_capabilities: AdvancedQueryCapabilities,
    pub allow_geometry_updates: bool,
    pub capabilities: String,
    pub copyright_text: String,
    pub current_version: f64,
    pub default_visibility: bool,
    pub drawing_info: DrawingInfo,
    pub edit_fields_info: Value,
    #[serde(rename = "enableZDefaults")]
    pub enable_zdefaults: bool,
    pub fields: Vec<Field>,
    pub geometry_type: String,
    pub global_id_field: String,
    pub has_attachments: bool,
    pub has_m: bool,
    pub has_static_data: bool,
    pub has_z: bool,
    pub html_popup_type: String,
    pub is_data_versioned: bool,
    pub max_record_count: i64,
    pub object_id_field: String,
    pub relationships: Vec<Value>,
    pub supported_query_formats: String,
    pub supports_advanced_queries: bool,
    pub supports_apply_edits_with_global_ids: bool,
    pub supports_attachments_by_upload_id: bool,
    pub supports_calculate: bool,
    pub supports_coordinates_quantization: bool,
    pub supports_rollback_on_failure_parameter: bool,
    #[serde(rename = "supportsValidateSQL")]
    pub supports_validate_sql: bool,
    pub templates: Vec<Template>,
    pub type_id_field: String,
    pub types: Vec<Value>,
    pub use_standardized_queries: bool,
    pub z_default: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedQueryCapabilities {
    pub supports_distinct: bool,
    pub supports_order_by: bool,
    pub supports_pagination: bool,
    pub supports_query_with_distance: bool,
    pub supports_returning_query_extent: bool,
    pub supports_statistics: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawingInfo {
    pub renderer: Renderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Renderer {
    pub description: String,
    pub label: String,
    pub symbol: Symbol,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub angle: i64,
    pub color: Vec<i64>,
    pub outline: Outline,
    pub size: i64,
    pub style: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub xoffset: i64,
    pub yoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outline {
    pub color: Vec<i64>,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub alias: String,
    pub sql_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub name: String,
    pub description: String,
    pub drawing_tool: String,
    pub prototype: Prototype,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prototype {
    pub attributes: HashMap<String, Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceSR {
    pub wkid: i64,
    pub latest_wkid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetSR {
    pub wkid: i64,
    pub latest_wkid: i64,
}

impl CSVPublishParameterBuilder {
    /// Create a new CSV publish parameter builder with sensible defaults
    ///
    /// # Arguments
    /// * `name` - The service name for the published feature service
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            type_field: "csv".to_string(),
            name: name.into(),
            source_url: Some("".to_string()),
            location_type: LocationType::Coordinates,
            latitude_field_name: None,
            longitude_field_name: None,
            coordinate_field_type: Some(CoordinateFieldType::LatitudeAndLongitude),
            layer_info: None,
            description: None,
            max_record_count: 1000,
            column_names: None,
            column_delimiter: Some(",".to_string()),
            source_sr: Some(SourceSR {
                wkid: 4326,
                latest_wkid: 4326,
            }),
            target_sr: Some(TargetSR {
                wkid: 102100,
                latest_wkid: 3857,
            }),
            editor_tracking_info: Some(EditorTrackingInfo {
                enable_editor_tracking: false,
                enable_ownership_access_control: false,
                allow_others_to_update: true,
                allow_others_to_delete: false,
            }),
            date_fields_time_reference: Some(DateFieldsTimeReference {
                time_zone: "UTC".to_string(),
            }),
            capabilities: Some(Capabilities::Query),
            has_static_data: Some(true),
            persist_error_records_for_review: Some(true),
        }
    }

    /// Set the coordinate field names for latitude and longitude
    ///
    /// # Arguments
    /// * `lat_field` - Name of the latitude field in the CSV
    /// * `lon_field` - Name of the longitude field in the CSV
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .set_coordinate_fields("Latitude", "Longitude");
    /// ```
    pub fn set_coordinate_fields(
        mut self,
        lat_field: impl Into<String>,
        lon_field: impl Into<String>,
    ) -> Self {
        self.latitude_field_name = Some(lat_field.into());
        self.longitude_field_name = Some(lon_field.into());
        self
    }

    /// Add a string field to the layer
    ///
    /// # Arguments
    /// * `name` - Name of the field
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .add_string_field("status");
    /// ```
    pub fn add_string_field(self, name: impl Into<String>) -> Self {
        self.add_field(name, "esriFieldTypeString", "sqlTypeOther", None, Some(256))
    }

    /// Add a double field to the layer
    ///
    /// # Arguments
    /// * `name` - Name of the field
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .add_double_field("temperature");
    /// ```
    pub fn add_double_field(self, name: impl Into<String>) -> Self {
        self.add_field(name, "esriFieldTypeDouble", "sqlTypeDouble", None, None)
    }

    /// Add an integer field to the layer
    ///
    /// # Arguments
    /// * `name` - Name of the field
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .add_integer_field("count");
    /// ```
    pub fn add_integer_field(self, name: impl Into<String>) -> Self {
        self.add_field(name, "esriFieldTypeInteger", "sqlTypeInteger", None, None)
    }

    /// Add a date field to the layer
    ///
    /// # Arguments
    /// * `name` - Name of the field
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .add_date_field("timestamp");
    /// ```
    pub fn add_date_field(self, name: impl Into<String>) -> Self {
        self.add_field(name, "esriFieldTypeDate", "sqlTypeTimestamp2", None, None)
    }

    /// Internal helper to add a field to the layer info
    fn add_field(
        mut self,
        name: impl Into<String>,
        field_type: &str,
        sql_type: &str,
        location_type: Option<String>,
        length: Option<i64>,
    ) -> Self {
        let name = name.into();
        let field = Field {
            name: name.clone(),
            type_field: field_type.to_string(),
            alias: name.clone(),
            sql_type: sql_type.to_string(),
            location_type,
            length,
        };

        // Initialize layer_info if not present
        if self.layer_info.is_none() {
            self.layer_info = Some(self.create_default_layer_info());
        }

        // Add field to layer_info
        if let Some(ref mut layer_info) = self.layer_info {
            layer_info.fields.push(field);
        }

        self
    }

    /// Set the maximum number of records that can be returned in a single query
    ///
    /// # Arguments
    /// * `max` - Maximum record count
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .set_max_records(2000);
    /// ```
    pub fn set_max_records(mut self, max: i64) -> Self {
        self.max_record_count = max;

        // Also update layer_info if present
        if let Some(ref mut layer_info) = self.layer_info {
            layer_info.max_record_count = max;
        }

        self
    }

    /// Set the description for the service
    ///
    /// # Arguments
    /// * `desc` - Service description
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .set_description("My CSV service");
    /// ```
    pub fn set_description(mut self, desc: impl Into<String>) -> Self {
        let desc = desc.into();
        self.description = Some(desc.clone());

        // Also update layer_info if present
        if let Some(ref mut layer_info) = self.layer_info {
            layer_info.description = desc;
        }

        self
    }

    /// Set the column delimiter used in the CSV file
    ///
    /// # Arguments
    /// * `delimiter` - Column delimiter (e.g., ",", ";", "\t")
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .set_column_delimiter(";");
    /// ```
    pub fn set_column_delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.column_delimiter = Some(delimiter.into());
        self
    }

    /// Set the source spatial reference
    ///
    /// # Arguments
    /// * `wkid` - Well-known ID of the spatial reference
    /// * `latest_wkid` - Latest well-known ID
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .set_source_sr(4326, 4326);
    /// ```
    pub fn set_source_sr(mut self, wkid: i64, latest_wkid: i64) -> Self {
        self.source_sr = Some(SourceSR { wkid, latest_wkid });
        self
    }

    /// Set the target spatial reference
    ///
    /// # Arguments
    /// * `wkid` - Well-known ID of the spatial reference
    /// * `latest_wkid` - Latest well-known ID
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .set_target_sr(102100, 3857);
    /// ```
    pub fn set_target_sr(mut self, wkid: i64, latest_wkid: i64) -> Self {
        self.target_sr = Some(TargetSR { wkid, latest_wkid });
        self
    }

    /// Enable or disable editor tracking
    ///
    /// # Arguments
    /// * `enable` - Whether to enable editor tracking
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .enable_editor_tracking(true);
    /// ```
    pub fn enable_editor_tracking(mut self, enable: bool) -> Self {
        if let Some(ref mut info) = self.editor_tracking_info {
            info.enable_editor_tracking = enable;
        }
        self
    }

    /// Set whether others can update features
    ///
    /// # Arguments
    /// * `allow` - Whether to allow others to update
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .allow_others_to_update(false);
    /// ```
    pub fn allow_others_to_update(mut self, allow: bool) -> Self {
        if let Some(ref mut info) = self.editor_tracking_info {
            info.allow_others_to_update = allow;
        }
        self
    }

    /// Set whether others can delete features
    ///
    /// # Arguments
    /// * `allow` - Whether to allow others to delete
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .allow_others_to_delete(true);
    /// ```
    pub fn allow_others_to_delete(mut self, allow: bool) -> Self {
        if let Some(ref mut info) = self.editor_tracking_info {
            info.allow_others_to_delete = allow;
        }
        self
    }

    /// Set the layer name (used in layerInfo)
    ///
    /// # Arguments
    /// * `layer_name` - Name for the layer
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::CSVPublishParameterBuilder;
    /// let builder = CSVPublishParameterBuilder::new("MyService")
    ///     .set_layer_name("MyLayer");
    /// ```
    pub fn set_layer_name(mut self, layer_name: impl Into<String>) -> Self {
        // Initialize layer_info if not present
        if self.layer_info.is_none() {
            self.layer_info = Some(self.create_default_layer_info());
        }

        if let Some(ref mut layer_info) = self.layer_info {
            layer_info.name = layer_name.into();
        }

        self
    }

    /// Create default layer info structure
    fn create_default_layer_info(&self) -> LayerInfo {
        LayerInfo {
            current_version: 11.3,
            id: 0,
            name: self.name.clone(),
            type_field: "Feature Layer".to_string(),
            display_field: "".to_string(),
            description: self.description.clone().unwrap_or_default(),
            copyright_text: "".to_string(),
            default_visibility: true,
            edit_fields_info: Value::Null,
            relationships: vec![],
            is_data_versioned: false,
            supports_rollback_on_failure_parameter: true,
            supports_advanced_queries: true,
            supports_validate_sql: true,
            supports_calculate: true,
            advanced_query_capabilities: AdvancedQueryCapabilities {
                supports_returning_query_extent: true,
                supports_statistics: true,
                supports_distinct: true,
                supports_pagination: true,
                supports_order_by: true,
                supports_query_with_distance: true,
            },
            geometry_type: "esriGeometryPoint".to_string(),
            drawing_info: DrawingInfo {
                renderer: Renderer {
                    type_field: "simple".to_string(),
                    symbol: Symbol {
                        type_field: "esriSMS".to_string(),
                        style: "esriSMSCircle".to_string(),
                        color: vec![129, 140, 0, 255],
                        size: 4,
                        angle: 0,
                        xoffset: 0,
                        yoffset: 0,
                        outline: Outline {
                            color: vec![0, 0, 0, 255],
                            width: 1,
                        },
                    },
                    label: "".to_string(),
                    description: "".to_string(),
                },
            },
            has_m: false,
            has_z: false,
            allow_geometry_updates: true,
            has_attachments: false,
            html_popup_type: "esriServerHTMLPopupTypeNone".to_string(),
            supports_apply_edits_with_global_ids: true,
            object_id_field: "".to_string(),
            global_id_field: "".to_string(),
            type_id_field: "".to_string(),
            fields: vec![],
            types: vec![],
            templates: vec![],
            use_standardized_queries: true,
            enable_zdefaults: false,
            z_default: 0,
            supported_query_formats: "JSON".to_string(),
            has_static_data: true,
            max_record_count: self.max_record_count,
            capabilities: "Query".to_string(),
            supports_coordinates_quantization: false,
            supports_attachments_by_upload_id: true,
        }
    }

    /// Build the final publish parameters as JSON
    ///
    /// This method is called internally to generate the complete JSON structure
    /// needed for publishing the CSV file.
    pub(crate) fn build(mut self) -> Value {
        // Initialize layer_info if not present
        if self.layer_info.is_none() {
            self.layer_info = Some(self.create_default_layer_info());
        }

        // Add coordinate fields to the field list if they're specified
        if let (Some(ref lon_field), Some(ref lat_field)) =
            (&self.longitude_field_name, &self.latitude_field_name)
        {
            if let Some(ref mut layer_info) = self.layer_info {
                // Check if coordinate fields aren't already in the list
                let has_lon = layer_info.fields.iter().any(|f| &f.name == lon_field);
                let has_lat = layer_info.fields.iter().any(|f| &f.name == lat_field);

                // Add longitude field if not present
                if !has_lon {
                    layer_info.fields.insert(
                        0,
                        Field {
                            name: lon_field.clone(),
                            type_field: "esriFieldTypeDouble".to_string(),
                            alias: lon_field.clone(),
                            location_type: Some("longitude".to_string()),
                            sql_type: "sqlTypeDouble".to_string(),
                            length: None,
                        },
                    );
                }

                // Add latitude field if not present
                if !has_lat {
                    layer_info.fields.insert(
                        if has_lon { 1 } else { 0 },
                        Field {
                            name: lat_field.clone(),
                            type_field: "esriFieldTypeDouble".to_string(),
                            alias: lat_field.clone(),
                            location_type: Some("latitude".to_string()),
                            sql_type: "sqlTypeDouble".to_string(),
                            length: None,
                        },
                    );
                }
            }
        }

        // Build the attributes map for templates (all fields with null values)
        let mut attributes = HashMap::new();
        if let Some(ref layer_info) = self.layer_info {
            for field in &layer_info.fields {
                attributes.insert(field.name.clone(), Value::Null);
            }
        }

        // Create the template with dynamic attributes
        if let Some(ref mut layer_info) = self.layer_info {
            layer_info.templates = vec![Template {
                name: "New Feature".to_string(),
                description: "".to_string(),
                drawing_tool: "esriFeatureEditToolPoint".to_string(),
                prototype: Prototype { attributes },
            }];
        }

        // Serialize to JSON value
        serde_json::to_value(&self).unwrap_or_else(|e| {
            eprintln!("Failed to serialize CSVPublishParameterBuilder: {}", e);
            Value::Null
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let builder = CSVPublishParameterBuilder::new("TestService")
            .set_coordinate_fields("Latitude", "Longitude");

        let json = builder.build();

        // Verify basic structure
        assert_eq!(json["type"], "csv");
        assert_eq!(json["name"], "TestService");
        assert_eq!(json["locationType"], "coordinates");
        assert_eq!(json["latitudeFieldName"], "Latitude");
        assert_eq!(json["longitudeFieldName"], "Longitude");
        assert_eq!(json["coordinateFieldType"], "LatitudeAndLongitude");

        // Verify spatial references
        assert_eq!(json["sourceSR"]["wkid"], 4326);
        assert_eq!(json["targetSR"]["wkid"], 102100);

        // Verify layer info exists
        assert!(json["layerInfo"].is_object());

        // Verify coordinate fields are in the field list
        let fields = json["layerInfo"]["fields"].as_array().unwrap();
        assert!(fields.iter().any(|f| f["name"] == "Longitude"));
        assert!(fields.iter().any(|f| f["name"] == "Latitude"));
    }

    #[test]
    fn test_builder_with_additional_fields() {
        let builder = CSVPublishParameterBuilder::new("TestService")
            .set_coordinate_fields("Latitude", "Longitude")
            .add_string_field("status")
            .add_double_field("temperature");

        let json = builder.build();

        let fields = json["layerInfo"]["fields"].as_array().unwrap();
        assert_eq!(fields.len(), 4); // Lat, Lon, status, temperature

        // Verify string field
        let status_field = fields.iter().find(|f| f["name"] == "status").unwrap();
        assert_eq!(status_field["type"], "esriFieldTypeString");
        assert_eq!(status_field["length"], 256);

        // Verify double field
        let temp_field = fields.iter().find(|f| f["name"] == "temperature").unwrap();
        assert_eq!(temp_field["type"], "esriFieldTypeDouble");

        // Verify templates include all fields
        let templates = json["layerInfo"]["templates"].as_array().unwrap();
        assert_eq!(templates.len(), 1);
        let attributes = &templates[0]["prototype"]["attributes"];
        assert!(attributes["Longitude"].is_null());
        assert!(attributes["Latitude"].is_null());
        assert!(attributes["status"].is_null());
        assert!(attributes["temperature"].is_null());
    }

    #[test]
    fn test_builder_customization() {
        let builder = CSVPublishParameterBuilder::new("TestService")
            .set_coordinate_fields("Latitude", "Longitude")
            .set_max_records(2000)
            .set_description("Test description")
            .set_column_delimiter(";")
            .set_source_sr(4269, 4269)
            .set_target_sr(3857, 3857);

        let json = builder.build();

        assert_eq!(json["maxRecordCount"], 2000);
        assert_eq!(json["description"], "Test description");
        assert_eq!(json["columnDelimiter"], ";");
        assert_eq!(json["sourceSR"]["wkid"], 4269);
        assert_eq!(json["targetSR"]["wkid"], 3857);
        assert_eq!(json["layerInfo"]["maxRecordCount"], 2000);
        assert_eq!(json["layerInfo"]["description"], "Test description");
    }
}
