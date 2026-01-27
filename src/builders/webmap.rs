use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::models::webmap::*;

/// Builder for creating web map JSON configurations
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebMapBuilder {
    operational_layers: Vec<OperationalLayer>,
    base_map: BaseMap,
    authoring_app: String,
    authoring_app_version: String,
    initial_state: Option<InitialState>,
    spatial_reference: SpatialReference,
    time_zone: String,
    version: String,
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
                drawing_info: Some(WebMapDrawingInfo::default()),
                definition_expression: Some(Value::Null),
            }),
            popup_info: None,
            item_id: None,
            feature_effect: Some(Value::Null),
            show_labels: Some(false),
            effect: None,
            blend_mode: None,
            style_url: None,
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
                drawing_info: Some(WebMapDrawingInfo::default()),
                definition_expression: Some(Value::Null),
            }),
            popup_info: None,
            item_id: Some(item_id),
            feature_effect: Some(Value::Null),
            show_labels: Some(false),
            effect: None,
            blend_mode: None,
            style_url: None,
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

    // pub fn build(self) -> WebMapJson {
    //     WebMapJson {
    //         operational_layers: self.operational_layers,
    //         base_map: self.basemap,
    //         authoring_app: self.authoring_app,
    //         authoring_app_version: self.authoring_app_version,
    //         initial_state: self.initial_state.unwrap_or_else(|| InitialState {
    //             viewpoint: Viewpoint {
    //                 target_geometry: TargetGeometry {
    //                     spatial_reference: SpatialReference {
    //                         latest_wkid: 3857,
    //                         wkid: 102100,
    //                     },
    //                     xmin: -20037508.342789244,
    //                     ymin: -20037508.342789244,
    //                     xmax: 20037508.342789244,
    //                     ymax: 20037508.342789244,
    //                 },
    //             },
    //         }),
    //         spatial_reference: self.spatial_reference,
    //         time_zone: self.time_zone,
    //         version: self.version,
    //     }
    // }

    /// Create a basemap configuration from a preset
    fn create_basemap_config(preset: BasemapPreset) -> BaseMap {
        match preset {
            BasemapPreset::Topographic => BaseMap {
                base_map_layers: vec![
                    BaseMapLayer {
                        id: "World_Hillshade_3689".to_string(),
                        opacity: Some(1),
                        title: "World Hillshade".to_string(),
                        url: Some("https://services.arcgisonline.com/arcgis/rest/services/Elevation/World_Hillshade/MapServer".to_string()),
                        visibility: true,
                        layer_type: "ArcGISTiledMapServiceLayer".to_string(),
                        effect: Vec::new(),
                        style_url: None,
                        blend_mode: None,
                    },
                    BaseMapLayer {
                        id: "VectorTile_6451".to_string(),
                        opacity: Some(1),
                        title: "World Topographic Map".to_string(),
                        url: None,
                        visibility: true,
                        layer_type: "VectorTileLayer".to_string(),
                        effect: Vec::new(),
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
                        opacity: Some(1),
                        title: "World Street Map".to_string(),
                        url: Some("https://services.arcgisonline.com/ArcGIS/rest/services/World_Street_Map/MapServer".to_string()),
                        visibility: true,
                        layer_type: "ArcGISTiledMapServiceLayer".to_string(),
                        effect: Vec::new(),
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
                        opacity: Some(1),
                        title: "World Imagery".to_string(),
                        url: Some("https://services.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer".to_string()),
                        visibility: true,
                        layer_type: "ArcGISTiledMapServiceLayer".to_string(),
                        effect: Vec::new(),
                        style_url: None,
                        blend_mode: None,
                    },
                    BaseMapLayer {
                        id: "World_Boundaries_and_Places_5488".to_string(),
                        opacity: Some(1),
                        title: "World Boundaries and Places".to_string(),
                        url: None,
                        visibility: true,
                        layer_type: "VectorTileLayer".to_string(),
                        effect: Vec::new(),
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
                        opacity: Some(1),
                        title: "Dark Gray Canvas".to_string(),
                        url: None,
                        visibility: true,
                        layer_type: "VectorTileLayer".to_string(),
                        effect: Vec::new(),
                        style_url: Some("https://cdn.arcgis.com/sharing/rest/content/items/5e9b3685f4c24d8781073dd928ebda50/resources/styles/root.json".to_string()),
                        blend_mode: None,
                    },
                ],
                title: "Dark Gray Canvas".to_string(),
            },
            BasemapPreset::LightGray => BaseMap {
                base_map_layers: vec![
                    BaseMapLayer {
                        id: "VectorTile_Light_Gray_2827".to_string(),
                        opacity: Some(1),
                        title: "Light Gray Canvas".to_string(),
                        url: None,
                        visibility: true,
                        layer_type: "VectorTileLayer".to_string(),
                        effect: Vec::new(),
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
                        opacity: Some(1),
                        title: "World Navigation Map".to_string(),
                        url: None,
                        visibility: true,
                        layer_type: "VectorTileLayer".to_string(),
                        effect: Vec::new(),
                        style_url: Some("https://cdn.arcgis.com/sharing/rest/content/items/63c47b7177f946b49902c24129b87252/resources/styles/root.json".to_string()),
                        blend_mode: None,
                    },
                ],
                title: "Navigation".to_string(),
            },
        }
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
