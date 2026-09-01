use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::models::webmap::*;

/// Builder for creating web map JSON configurations
#[derive(Default, Debug, Clone, PartialEq)]
pub struct WebMapBuilder {
    operational_layers: Vec<OperationalLayer>,
    base_map: BaseMap,
    authoring_app: String,
    authoring_app_version: String,
    initial_state: Option<InitialState>,
    spatial_reference: SpatialReference,
    time_zone: String,
    version: String,
    renderer_overrides: BTreeMap<usize, Value>,
}

impl<'de> Deserialize<'de> for WebMapBuilder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct DeserializableWebMap {
            operational_layers: Vec<Value>,
            base_map: BaseMap,
            authoring_app: String,
            authoring_app_version: String,
            initial_state: Option<InitialState>,
            spatial_reference: SpatialReference,
            time_zone: String,
            version: String,
        }

        let raw = DeserializableWebMap::deserialize(deserializer)?;
        let mut renderer_overrides = BTreeMap::new();
        let operational_layers = raw
            .operational_layers
            .into_iter()
            .enumerate()
            .map(|(index, mut value)| {
                if let Some(renderer) = value.pointer("/layerDefinition/drawingInfo/renderer") {
                    renderer_overrides.insert(index, renderer.clone());
                    value["layerDefinition"]["drawingInfo"] = Value::Null;
                }
                serde_json::from_value(value).map_err(serde::de::Error::custom)
            })
            .collect::<Result<Vec<_>, D::Error>>()?;

        Ok(Self {
            operational_layers,
            base_map: raw.base_map,
            authoring_app: raw.authoring_app,
            authoring_app_version: raw.authoring_app_version,
            initial_state: raw.initial_state,
            spatial_reference: raw.spatial_reference,
            time_zone: raw.time_zone,
            version: raw.version,
            renderer_overrides,
        })
    }
}

impl Serialize for WebMapBuilder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct SerializableWebMap<'a> {
            operational_layers: Vec<Value>,
            base_map: &'a BaseMap,
            authoring_app: &'a str,
            authoring_app_version: &'a str,
            initial_state: &'a Option<InitialState>,
            spatial_reference: &'a SpatialReference,
            time_zone: &'a str,
            version: &'a str,
        }

        let operational_layers = self
            .operational_layers
            .iter()
            .enumerate()
            .map(|(index, layer)| {
                let mut value = serde_json::to_value(layer).map_err(serde::ser::Error::custom)?;
                if let Some(renderer) = self.renderer_overrides.get(&index) {
                    value["layerDefinition"]["drawingInfo"] = serde_json::json!({
                        "renderer": renderer
                    });
                }
                Ok(value)
            })
            .collect::<Result<Vec<_>, S::Error>>()?;

        SerializableWebMap {
            operational_layers,
            base_map: &self.base_map,
            authoring_app: &self.authoring_app,
            authoring_app_version: &self.authoring_app_version,
            initial_state: &self.initial_state,
            spatial_reference: &self.spatial_reference,
            time_zone: &self.time_zone,
            version: &self.version,
        }
        .serialize(serializer)
    }
}

impl WebMapBuilder {
    /// Create a new web map builder with sensible defaults
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::BasemapPreset;
    /// # use arcgis_sharing_rs::builders::WebMapBuilder;
    /// let web_map = WebMapBuilder::new()
    ///     .add_feature_layer("https://services.arcgis.com/.../FeatureServer/0", "My Layer")
    ///     .set_basemap(BasemapPreset::Topographic);
    /// ```
    pub fn new() -> Self {
        Self {
            operational_layers: Vec::new(),
            base_map: Self::create_basemap_config(BasemapPreset::Topographic),
            authoring_app: "ArcGISMapViewer".to_string(),
            authoring_app_version: "2025.3".to_string(),
            initial_state: None,
            spatial_reference: SpatialReference {
                latest_wkid: 3857,
                wkid: 102100,
            },
            time_zone: "system".to_string(),
            version: "2.35".to_string(),
            renderer_overrides: BTreeMap::new(),
        }
    }

    /// Add a feature layer to the web map
    ///
    /// # Arguments
    /// * `url` - URL to the feature service layer
    /// * `title` - Display title for the layer
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::builders::WebMapBuilder;
    /// let builder = WebMapBuilder::new()
    ///     .add_feature_layer("https://services.arcgis.com/.../FeatureServer/0", "My Layer");
    /// ```
    pub fn add_feature_layer(mut self, url: impl Into<String>, title: impl Into<String>) -> Self {
        let url = url.into();
        let title = title.into();
        let id = format!("layer-{}", self.operational_layers.len());

        let layer = OperationalLayer {
            id,
            show_legend: Some(true),
            opacity: Some(1.0),
            disable_popup: Some(false),
            title,
            url: Some(url),
            visibility: Some(true),
            layer_type: "ArcGISFeatureLayer".to_string(),
            layer_definition: Some(LayerDefinition {
                feature_reduction: Some(Value::Null),
                drawing_info: None,
                definition_expression: Some(Value::Null),
            }),
            popup_info: None,
            item_id: None,
            //feature_effect: Some(Value::Null),
            show_labels: Some(false),
            // effect: None,
            // blend_mode: None,
            // style_url: None,
        };

        self.operational_layers.push(layer);
        self
    }

    /// Add a feature layer with an item ID reference
    ///
    /// # Arguments
    /// * `url` - URL to the feature service layer
    /// * `title` - Display title for the layer
    /// * `item_id` - Item ID of the feature service
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::builders::WebMapBuilder;
    /// let builder = WebMapBuilder::new()
    ///     .add_feature_layer_with_item_id(
    ///         "https://services.arcgis.com/.../FeatureServer/0",
    ///         "My Layer",
    ///         "abc123"
    ///     );
    /// ```
    pub fn add_feature_layer_with_item_id(
        mut self,
        url: impl Into<String>,
        title: impl Into<String>,
        item_id: impl Into<String>,
    ) -> Self {
        let url = url.into();
        let title = title.into();
        let item_id = item_id.into();
        let id = format!("layer-{}", self.operational_layers.len());

        let layer = OperationalLayer {
            id,
            show_legend: Some(true),
            opacity: Some(1.0),
            disable_popup: Some(false),
            title,
            url: Some(url),
            visibility: Some(true),
            layer_type: "ArcGISFeatureLayer".to_string(),
            layer_definition: Some(LayerDefinition {
                feature_reduction: Some(Value::Null),
                drawing_info: None,
                definition_expression: Some(Value::Null),
            }),
            popup_info: None,
            item_id: Some(item_id),
            //feature_effect: Some(Value::Null),
            show_labels: Some(false),
            // effect: None,
            // blend_mode: None,
            // style_url: None,
        };

        self.operational_layers.push(layer);
        self
    }

    /// Set visibility for the last added layer
    ///
    /// # Arguments
    /// * `visible` - Whether the layer should be visible
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::builders::WebMapBuilder;
    /// let builder = WebMapBuilder::new()
    ///     .add_feature_layer("https://services.arcgis.com/.../FeatureServer/0", "My Layer")
    ///     .set_layer_visibility(false);
    /// ```
    pub fn set_layer_visibility(mut self, visible: bool) -> Self {
        if let Some(layer) = self.operational_layers.last_mut() {
            layer.visibility = Some(visible);
        }
        self
    }

    /// Set opacity for the last added layer
    ///
    /// # Arguments
    /// * `opacity` - Opacity value between 0.0 and 1.0
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::builders::WebMapBuilder;
    /// let builder = WebMapBuilder::new()
    ///     .add_feature_layer("https://services.arcgis.com/.../FeatureServer/0", "My Layer")
    ///     .set_layer_opacity(0.75);
    /// ```
    pub fn set_layer_opacity(mut self, opacity: f64) -> Self {
        if let Some(layer) = self.operational_layers.last_mut() {
            layer.opacity = Some(opacity);
        }
        self
    }

    /// Override symbology for the last added layer with a simple renderer.
    pub fn with_layer_symbology(mut self, color: [u8; 4], geometry_type: &str) -> Self {
        if let Some(index) = self.operational_layers.len().checked_sub(1) {
            self.renderer_overrides.remove(&index);
        }
        if let Some(layer) = self.operational_layers.last_mut() {
            if let Some(ref mut layer_def) = layer.layer_definition {
                layer_def.drawing_info = Some(simple_drawing_info(color, geometry_type));
            }
        }
        self
    }

    /// Override symbology for the last added layer with a native ArcGIS renderer.
    pub fn with_layer_renderer(mut self, renderer: Value) -> Self {
        if let Some(index) = self.operational_layers.len().checked_sub(1) {
            self.renderer_overrides.insert(index, renderer);
        }
        self
    }

    /// Enable popup for the last added layer
    ///
    /// # Arguments
    /// * `title` - Popup title (use '{field}' to reference a field)
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::builders::WebMapBuilder;
    /// let builder = WebMapBuilder::new()
    ///     .add_feature_layer("https://services.arcgis.com/.../FeatureServer/0", "My Layer")
    ///     .with_popup("Feature Info {objectid}");
    /// ```
    pub fn with_popup(mut self, title: impl Into<String>) -> Self {
        if let Some(layer) = self.operational_layers.last_mut() {
            layer.popup_info = Some(PopupInfo {
                popup_elements: vec![PopupElement {
                    type_field: "fields".to_string(),
                    text: None,
                    description: None,
                    field_infos: Vec::new(),
                    title: None,
                }],
                description: None,
                expression_infos: Some(Vec::new()),
                field_infos: Vec::new(),
                title: title.into(),
            });
        }
        self
    }

    /// Add a field to the popup of the last added layer
    ///
    /// # Arguments
    /// * `field_name` - Name of the field
    /// * `label` - Display label
    /// * `editable` - Whether the field is editable
    /// * `visible` - Whether the field is visible
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::builders::WebMapBuilder;
    /// let builder = WebMapBuilder::new()
    ///     .add_feature_layer("https://services.arcgis.com/.../FeatureServer/0", "My Layer")
    ///     .with_popup("Feature Info")
    ///     .add_popup_field("name", "Name", true, true);
    /// ```
    pub fn add_popup_field(
        mut self,
        field_name: impl Into<String>,
        label: impl Into<String>,
        editable: bool,
        visible: bool,
    ) -> Self {
        if let Some(layer) = self.operational_layers.last_mut() {
            if let Some(ref mut popup_info) = layer.popup_info {
                let field_name = field_name.into();
                let label = label.into();

                let field_info = FieldInfo {
                    field_name: field_name.clone(),
                    format: None,
                    is_editable: editable,
                    label: label.clone(),
                    visible,
                };
                popup_info.field_infos.push(field_info);

                // Also add to popup element field infos
                if let Some(element) = popup_info.popup_elements.first_mut() {
                    element.field_infos.push(FieldInfo {
                        field_name,
                        is_editable: editable,
                        label,
                        visible,
                        format: None,
                    });
                }
            }
        }
        self
    }

    /// Add a field with number formatting to the popup of the last added layer
    ///
    /// # Arguments
    /// * `field_name` - Name of the field
    /// * `label` - Display label
    /// * `editable` - Whether the field is editable
    /// * `visible` - Whether the field is visible
    /// * `places` - Number of decimal places
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::builders::WebMapBuilder;
    /// let builder = WebMapBuilder::new()
    ///     .add_feature_layer("https://services.arcgis.com/.../FeatureServer/0", "My Layer")
    ///     .with_popup("Feature Info")
    ///     .add_popup_field_with_format("temperature", "Temperature", true, true, 2);
    /// ```
    pub fn add_popup_field_with_format(
        mut self,
        field_name: impl Into<String>,
        label: impl Into<String>,
        editable: bool,
        visible: bool,
        places: i64,
    ) -> Self {
        if let Some(layer) = self.operational_layers.last_mut() {
            if let Some(ref mut popup_info) = layer.popup_info {
                let field_name = field_name.into();
                let label = label.into();

                let format = Format {
                    digit_separator: true,
                    places: Some(places),
                };

                let field_info = FieldInfo {
                    field_name: field_name.clone(),
                    format: Some(format.clone()),
                    is_editable: editable,
                    label: label.clone(),
                    visible,
                };
                popup_info.field_infos.push(field_info);

                // Also add to popup element field infos
                if let Some(element) = popup_info.popup_elements.first_mut() {
                    element.field_infos.push(FieldInfo {
                        field_name,
                        is_editable: editable,
                        label,
                        visible,
                        format: Some(Format {
                            digit_separator: true,
                            places: Some(places),
                        }),
                    });
                }
            }
        }
        self
    }

    /// Set the basemap using a preset
    ///
    /// # Arguments
    /// * `preset` - Basemap preset to use
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::BasemapPreset;
    /// # use arcgis_sharing_rs::builders::WebMapBuilder;
    /// let builder = WebMapBuilder::new()
    ///     .set_basemap(BasemapPreset::Streets);
    /// ```
    pub fn set_basemap(mut self, preset: BasemapPreset) -> Self {
        self.base_map = Self::create_basemap_config(preset);
        self
    }

    /// Set the initial extent of the web map
    ///
    /// # Arguments
    /// * `xmin` - Minimum X coordinate
    /// * `ymin` - Minimum Y coordinate
    /// * `xmax` - Maximum X coordinate
    /// * `ymax` - Maximum Y coordinate
    /// * `wkid` - Well-known ID of the spatial reference
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::builders::WebMapBuilder;
    /// let builder = WebMapBuilder::new()
    ///     .set_extent(-109.5, 41.0, -109.0, 41.5, 4326);
    /// ```
    pub fn set_extent(mut self, xmin: f64, ymin: f64, xmax: f64, ymax: f64, wkid: i64) -> Self {
        self.initial_state = Some(InitialState {
            viewpoint: Some(Viewpoint {
                target_geometry: TargetGeometry {
                    spatial_reference: SpatialReference {
                        latest_wkid: wkid,
                        wkid,
                    },
                    xmin,
                    ymin,
                    xmax,
                    ymax,
                },
            }),
        });
        self
    }

    /// Create a basemap configuration from a preset
    fn create_basemap_config(preset: BasemapPreset) -> BaseMap {
        match preset {
            BasemapPreset::Topographic => BaseMap {
                base_map_layers: vec![
                    BaseMapLayer {
                        id: "World_Hillshade_3689".to_string(),
                        opacity: Some(1.0),
                        title: "World Hillshade".to_string(),
                        item_id: None,
                        is_reference: None,
                        url: Some("https://services.arcgisonline.com/arcgis/rest/services/Elevation/World_Hillshade/MapServer".to_string()),
                        visibility: true,
                        layer_type: "ArcGISTiledMapServiceLayer".to_string(),
                        effect: Some(Vec::new()),
                        style_url: None,
                        blend_mode: None,
                    },
                    BaseMapLayer {
                        id: "VectorTile_6451".to_string(),
                        opacity: Some(1.0),
                        title: "World Topographic Map".to_string(),
                        item_id: None,
                        is_reference: None,
                        url: None,
                        visibility: true,
                        layer_type: "VectorTileLayer".to_string(),
                        effect: Some(Vec::new()),
                        style_url: Some("https://cdn.arcgis.com/sharing/rest/content/items/7dc6cea0b1764a1f9af2e679f642f0f5/resources/styles/root.json".to_string()),
                        blend_mode: None,
                    },
                ],
                title: "Topographic".to_string(),
            },
            BasemapPreset::Streets => BaseMap {
                base_map_layers: vec![
                    BaseMapLayer {
                        id: "World_Street_Map_8722".to_string(),
                        opacity: Some(1.0),
                        title: "World Street Map".to_string(),
                        item_id: None,
                        is_reference: None,
                        url: Some("https://services.arcgisonline.com/ArcGIS/rest/services/World_Street_Map/MapServer".to_string()),
                        visibility: true,
                        layer_type: "ArcGISTiledMapServiceLayer".to_string(),
                        effect: Some(Vec::new()),
                        style_url: None,
                        blend_mode: None,
                    },
                ],
                title: "Streets".to_string(),
            },
            BasemapPreset::Imagery => BaseMap {
                base_map_layers: vec![
                    BaseMapLayer {
                        id: "World_Imagery_2233".to_string(),
                        opacity: Some(1.0),
                        title: "World Imagery".to_string(),
                        item_id: None,
                        is_reference: None,
                        url: Some("https://services.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer".to_string()),
                        visibility: true,
                        layer_type: "ArcGISTiledMapServiceLayer".to_string(),
                        effect: Some(Vec::new()),
                        style_url: None,
                        blend_mode: None,
                    },
                    BaseMapLayer {
                        id: "World_Boundaries_and_Places_5488".to_string(),
                        opacity: Some(1.0),
                        title: "World Boundaries and Places".to_string(),
                        item_id: None,
                        is_reference: None,
                        url: None,
                        visibility: true,
                        layer_type: "VectorTileLayer".to_string(),
                        effect: Some(Vec::new()),
                        style_url: Some("https://cdn.arcgis.com/sharing/rest/content/items/2afe5b807fa74006be6363fd243ffb30/resources/styles/root.json".to_string()),
                        blend_mode: None,
                    },
                ],
                title: "Imagery".to_string(),
            },
            BasemapPreset::DarkGray => BaseMap {
                base_map_layers: vec![
                    BaseMapLayer {
                        id: "VectorTile_Dark_Gray_8199".to_string(),
                        opacity: Some(1.0),
                        title: "Dark Gray Canvas Base".to_string(),
                        item_id: Some("5e9b3685f4c24d8781073dd928ebda50".to_string()),
                        is_reference: None,
                        url: None,
                        visibility: true,
                        layer_type: "VectorTileLayer".to_string(),
                        effect: Some(Vec::new()),
                        style_url: Some("https://cdn.arcgis.com/sharing/rest/content/items/5e9b3685f4c24d8781073dd928ebda50/resources/styles/root.json".to_string()),
                        blend_mode: None,
                    },
                    BaseMapLayer {
                        id: "VectorTile_Dark_Gray_Reference_747c".to_string(),
                        opacity: Some(1.0),
                        title: "Dark Gray Canvas Reference".to_string(),
                        item_id: Some("747cb7a5329c478cbe6981076cc879c5".to_string()),
                        is_reference: Some(true),
                        url: None,
                        visibility: true,
                        layer_type: "VectorTileLayer".to_string(),
                        effect: Some(Vec::new()),
                        style_url: Some("https://cdn.arcgis.com/sharing/rest/content/items/747cb7a5329c478cbe6981076cc879c5/resources/styles/root.json".to_string()),
                        blend_mode: None,
                    },
                ],
                title: "Dark Gray Canvas".to_string(),
            },
            BasemapPreset::LightGray => BaseMap {
                base_map_layers: vec![
                    BaseMapLayer {
                        id: "VectorTile_Light_Gray_2827".to_string(),
                        opacity: Some(1.0),
                        title: "Light Gray Canvas".to_string(),
                        item_id: None,
                        is_reference: None,
                        url: None,
                        visibility: true,
                        layer_type: "VectorTileLayer".to_string(),
                        effect: Some(Vec::new()),
                        style_url: Some("https://cdn.arcgis.com/sharing/rest/content/items/8a2cba3b0ebf4140b7c0dc5ee149549a/resources/styles/root.json".to_string()),
                        blend_mode: None,
                    },
                ],
                title: "Light Gray Canvas".to_string(),
            },
            BasemapPreset::Navigation => BaseMap {
                base_map_layers: vec![
                    BaseMapLayer {
                        id: "VectorTile_Navigation_8145".to_string(),
                        opacity: Some(1.0),
                        title: "World Navigation Map".to_string(),
                        item_id: None,
                        is_reference: None,
                        url: None,
                        visibility: true,
                        layer_type: "VectorTileLayer".to_string(),
                        effect: Some(Vec::new()),
                        style_url: Some("https://cdn.arcgis.com/sharing/rest/content/items/63c47b7177f946b49902c24129b87252/resources/styles/root.json".to_string()),
                        blend_mode: None,
                    },
                ],
                title: "Navigation".to_string(),
            },
        }
    }
}

fn color_vec(color: [u8; 4]) -> Vec<i64> {
    color.into_iter().map(i64::from).collect()
}

fn black_outline(width: f64) -> Outline {
    Outline {
        type_field: "esriSLS".to_string(),
        color: vec![0, 0, 0, 255],
        width,
        style: "esriSLSSolid".to_string(),
    }
}

fn simple_drawing_info(color: [u8; 4], geometry_type: &str) -> WebMapDrawingInfo {
    let rgba = color_vec(color);
    let symbol = match geometry_type {
        "esriGeometryPolyline" => WebMapSymbol {
            type_field: "esriSLS".to_string(),
            color: Some(rgba),
            style: Some("esriSLSSolid".to_string()),
            width: Some(2.0),
            outline: None,
            symbol: None,
            size: None,
        },
        "esriGeometryPolygon" => WebMapSymbol {
            type_field: "esriSFS".to_string(),
            color: Some(rgba),
            style: Some("esriSFSSolid".to_string()),
            outline: Some(black_outline(1.0)),
            symbol: None,
            size: None,
            width: None,
        },
        _ => WebMapSymbol {
            type_field: "esriSMS".to_string(),
            color: Some(rgba),
            style: Some("esriSMSCircle".to_string()),
            size: Some(6.0),
            outline: Some(black_outline(1.0)),
            symbol: None,
            width: None,
        },
    };

    WebMapDrawingInfo {
        renderer: WebMapRenderer {
            type_field: "simple".to_string(),
            symbol: Some(symbol),
            ..Default::default()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_map_builder_basic() {
        let web_map = WebMapBuilder::new()
            .add_feature_layer(
                "https://services.arcgis.com/test/FeatureServer/0",
                "Test Layer",
            )
            .set_basemap(BasemapPreset::Topographic);

        // Verify basic structure
        assert_eq!(web_map.operational_layers.len(), 1);
        assert_eq!(web_map.operational_layers[0].title, "Test Layer");
        assert_eq!(
            web_map.operational_layers[0].layer_type,
            "ArcGISFeatureLayer"
        );
        assert_eq!(web_map.base_map.title, "Topographic");
        assert_eq!(web_map.authoring_app, "ArcGISMapViewer");
        assert_eq!(web_map.version, "2.35");
    }

    #[test]
    fn test_web_map_builder_with_popup() {
        let web_map = WebMapBuilder::new()
            .add_feature_layer(
                "https://services.arcgis.com/test/FeatureServer/0",
                "Test Layer",
            )
            .with_popup("Feature Info")
            .add_popup_field("name", "Name", true, true)
            .add_popup_field_with_format("value", "Value", true, true, 2);

        // Verify popup configuration
        let layer = &web_map.operational_layers[0];
        assert!(layer.popup_info.is_some());

        let popup = layer.popup_info.as_ref().unwrap();
        assert_eq!(popup.title, "Feature Info");
        assert_eq!(popup.field_infos.len(), 2);
        assert_eq!(popup.field_infos[0].field_name, "name");
        assert_eq!(popup.field_infos[1].field_name, "value");
        assert!(popup.field_infos[1].format.is_some());
    }

    #[test]
    fn test_web_map_builder_basemap_presets() {
        let basemaps = vec![
            (BasemapPreset::Topographic, "Topographic"),
            (BasemapPreset::Streets, "Streets"),
            (BasemapPreset::Imagery, "Imagery"),
            (BasemapPreset::DarkGray, "Dark Gray Canvas"),
            (BasemapPreset::LightGray, "Light Gray Canvas"),
            (BasemapPreset::Navigation, "Navigation"),
        ];

        for (preset, expected_title) in basemaps {
            let web_map = WebMapBuilder::new().set_basemap(preset);
            assert_eq!(web_map.base_map.title, expected_title);
        }
    }

    #[test]
    fn dark_gray_basemap_includes_reference_layer() {
        let web_map = WebMapBuilder::new().set_basemap(BasemapPreset::DarkGray);

        assert_eq!(web_map.base_map.base_map_layers.len(), 2);
        assert_eq!(
            web_map.base_map.base_map_layers[1].title,
            "Dark Gray Canvas Reference"
        );
        assert_eq!(web_map.base_map.base_map_layers[1].is_reference, Some(true));

        let json = serde_json::to_value(web_map).unwrap();
        assert_eq!(json["baseMap"]["baseMapLayers"][1]["isReference"], true);
        assert_eq!(
            json["baseMap"]["baseMapLayers"][1]["itemId"],
            "747cb7a5329c478cbe6981076cc879c5"
        );
    }

    #[test]
    fn test_web_map_builder_extent() {
        let web_map = WebMapBuilder::new().set_extent(-109.5, 41.0, -109.0, 41.5, 4326);

        let viewpoint = &web_map.initial_state.unwrap().viewpoint.clone();
        let geom = &viewpoint.clone().unwrap().target_geometry;
        assert_eq!(geom.xmin, -109.5);
        assert_eq!(geom.ymin, 41.0);
        assert_eq!(geom.xmax, -109.0);
        assert_eq!(geom.ymax, 41.5);
        assert_eq!(geom.spatial_reference.wkid, 4326);
    }

    #[test]
    fn test_web_map_builder_layer_symbology() {
        let color = [31, 119, 180, 255];
        let web_map = WebMapBuilder::new()
            .add_feature_layer(
                "https://services.arcgis.com/test/FeatureServer/0",
                "Test Layer",
            )
            .with_layer_symbology(color, "esriGeometryPoint");

        let layer_def = web_map.operational_layers[0]
            .layer_definition
            .as_ref()
            .unwrap();
        let drawing_info = layer_def.drawing_info.as_ref().unwrap();
        let symbol = drawing_info.renderer.symbol.as_ref().unwrap();
        assert_eq!(symbol.type_field, "esriSMS");
        assert_eq!(symbol.color.as_ref().unwrap(), &color_vec(color));

        let json = serde_json::to_string(&web_map).unwrap();
        assert!(json.contains("drawingInfo"));
        assert!(json.contains("[31,119,180,255]"));
    }

    #[test]
    fn test_web_map_builder_generated_renderer() {
        let renderer = serde_json::json!({
            "type": "uniqueValue",
            "field1": "status",
            "uniqueValueInfos": []
        });
        let web_map = WebMapBuilder::new()
            .add_feature_layer(
                "https://services.arcgis.com/test/FeatureServer/0",
                "Test Layer",
            )
            .with_layer_renderer(renderer.clone());

        let json = serde_json::to_value(web_map).unwrap();
        assert_eq!(
            json["operationalLayers"][0]["layerDefinition"]["drawingInfo"]["renderer"],
            renderer
        );
    }

    #[test]
    fn test_web_map_serialization() {
        let web_map = WebMapBuilder::new()
            .add_feature_layer(
                "https://services.arcgis.com/test/FeatureServer/0",
                "Test Layer",
            )
            .set_basemap(BasemapPreset::Topographic);

        println!("{}", serde_json::to_string_pretty(&web_map).unwrap());

        // Verify it can be serialized to JSON
        let json = serde_json::to_string(&web_map).unwrap();
        assert!(json.contains("operationalLayers"));
        assert!(json.contains("baseMap"));
        assert!(json.contains("Test Layer"));
        assert!(json.contains("Topographic"));
    }
}
