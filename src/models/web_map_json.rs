use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebMapJson {
    pub operational_layers: Vec<OperationalLayer>,
    pub base_map: BaseMap,
    pub authoring_app: String,
    pub authoring_app_version: String,
    pub initial_state: InitialState,
    pub spatial_reference: SpatialReference2,
    pub time_zone: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationalLayer {
    pub id: String,
    pub show_legend: Option<bool>,
    pub opacity: Option<f64>,
    pub disable_popup: Option<bool>,
    pub title: String,
    pub url: Option<String>,
    pub visibility: Option<bool>,
    pub layer_type: String,
    pub layer_definition: Option<LayerDefinition>,
    pub popup_info: Option<PopupInfo>,
    pub item_id: Option<String>,
    pub feature_effect: Value,
    pub show_labels: Option<bool>,
    pub effect: Option<Vec<Effect>>,
    pub blend_mode: Option<String>,
    pub style_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerDefinition {
    pub feature_reduction: Value,
    pub drawing_info: WebMapDrawingInfo,
    pub definition_expression: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebMapDrawingInfo {
    pub renderer: WebMapRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebMapRenderer {
    #[serde(rename = "type")]
    pub type_field: String,
    pub symbol: Option<WebMapSymbol>,
    pub authoring_info: Option<AuthoringInfo>,
    #[serde(default)]
    pub class_break_infos: Vec<ClassBreakInfo>,
    pub field: Option<String>,
    pub legend_options: Option<LegendOptions>,
    pub min_value: Option<i64>,
    pub visual_variables: Option<Vec<VisualVariable2>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebMapSymbol {
    #[serde(rename = "type")]
    pub type_field: String,
    pub color: Option<Vec<i64>>,
    pub outline: Option<Outline>,
    pub style: Option<String>,
    pub symbol: Option<WebMapSymbol2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outline {
    #[serde(rename = "type")]
    pub type_field: String,
    pub color: Vec<i64>,
    pub width: f64,
    pub style: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebMapSymbol2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub symbol_layers: Vec<SymbolLayer>,
    pub angle_alignment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolLayer {
    #[serde(rename = "type")]
    pub type_field: String,
    pub enable: bool,
    pub cap_style: Option<String>,
    pub join_style: Option<String>,
    #[serde(rename = "lineStyle3D")]
    pub line_style3d: Option<String>,
    pub miter_limit: Option<i64>,
    pub width: Option<f64>,
    #[serde(rename = "height3D")]
    pub height3d: Option<i64>,
    #[serde(rename = "anchor3D")]
    pub anchor3d: Option<String>,
    pub color: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoringInfo {
    #[serde(default)]
    pub visual_variables: Vec<VisualVariable>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualVariable {
    pub max_slider_value: i64,
    pub min_slider_value: i64,
    pub theme: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassBreakInfo {
    pub class_max_value: i64,
    pub symbol: WebMapSymbol3,
    pub label: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebMapSymbol3 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub color: Vec<i64>,
    pub outline: Outline2,
    pub style: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outline2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub color: Vec<i64>,
    pub width: f64,
    pub style: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegendOptions {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualVariable2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub field: Option<String>,
    pub stops: Vec<Stop>,
    pub value_expression: Option<String>,
    pub target: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub color: Option<Vec<i64>>,
    pub value: i64,
    pub size: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PopupInfo {
    pub popup_elements: Vec<PopupElement>,
    pub description: Option<String>,
    pub expression_infos: Vec<Value>,
    pub field_infos: Vec<FieldInfo2>,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PopupElement {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub field_infos: Vec<FieldInfo>,
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInfo {
    pub field_name: String,
    pub is_editable: bool,
    pub label: String,
    pub visible: bool,
    pub format: Option<Format>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub digit_separator: bool,
    pub places: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInfo2 {
    pub field_name: String,
    pub format: Option<Format2>,
    pub is_editable: bool,
    pub label: String,
    pub visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format2 {
    pub digit_separator: bool,
    pub places: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub scale: f64,
    pub value: Vec<EffectValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectValue {
    #[serde(rename = "type")]
    pub type_field: String,
    pub xoffset: f64,
    pub yoffset: f64,
    pub blur_radius: f64,
    pub color: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseMap {
    pub base_map_layers: Vec<BaseMapLayer>,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseMapLayer {
    pub id: String,
    pub opacity: Option<i64>,
    pub title: String,
    pub url: Option<String>,
    pub visibility: bool,
    pub layer_type: String,
    #[serde(default)]
    pub effect: Vec<Effect2>,
    pub style_url: Option<String>,
    pub blend_mode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub angle: Option<i64>,
    pub amount: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialState {
    pub viewpoint: Viewpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewpoint {
    pub target_geometry: TargetGeometry,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetGeometry {
    pub spatial_reference: SpatialReference,
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialReference {
    pub latest_wkid: i64,
    pub wkid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialReference2 {
    pub latest_wkid: i64,
    pub wkid: i64,
}

/// Preset basemap options for web maps
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BasemapPreset {
    Topographic,
    Streets,
    Imagery,
    DarkGray,
    LightGray,
    Navigation,
}

/// Builder for creating web map JSON configurations
pub struct WebMapBuilder {
    operational_layers: Vec<OperationalLayer>,
    basemap: BaseMap,
    authoring_app: String,
    authoring_app_version: String,
    initial_state: Option<InitialState>,
    spatial_reference: SpatialReference2,
    time_zone: String,
    version: String,
}

impl WebMapBuilder {
    /// Create a new web map builder with sensible defaults
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::{WebMapBuilder, BasemapPreset};
    /// let web_map = WebMapBuilder::new()
    ///     .add_feature_layer("https://services.arcgis.com/.../FeatureServer/0", "My Layer")
    ///     .set_basemap(BasemapPreset::Topographic)
    ///     .build();
    /// ```
    pub fn new() -> Self {
        Self {
            operational_layers: Vec::new(),
            basemap: Self::create_basemap_config(BasemapPreset::Topographic),
            authoring_app: "ArcGISMapViewer".to_string(),
            authoring_app_version: "2025.3".to_string(),
            initial_state: None,
            spatial_reference: SpatialReference2 {
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
    /// # use arcgis_sharing_rs::models::WebMapBuilder;
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
                feature_reduction: Value::Null,
                drawing_info: WebMapDrawingInfo::default(),
                definition_expression: Value::Null,
            }),
            popup_info: None,
            item_id: None,
            feature_effect: Value::Null,
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
    /// # use arcgis_sharing_rs::models::WebMapBuilder;
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
                feature_reduction: Value::Null,
                drawing_info: WebMapDrawingInfo::default(),
                definition_expression: Value::Null,
            }),
            popup_info: None,
            item_id: Some(item_id),
            feature_effect: Value::Null,
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
    /// # use arcgis_sharing_rs::models::WebMapBuilder;
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
    /// # use arcgis_sharing_rs::models::WebMapBuilder;
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
    /// # use arcgis_sharing_rs::models::WebMapBuilder;
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
                expression_infos: Vec::new(),
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
    /// # use arcgis_sharing_rs::models::WebMapBuilder;
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

                let field_info = FieldInfo2 {
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
    /// # use arcgis_sharing_rs::models::WebMapBuilder;
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

                let format = Format2 {
                    digit_separator: true,
                    places: Some(places),
                };

                let field_info = FieldInfo2 {
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
    /// # use arcgis_sharing_rs::models::{WebMapBuilder, BasemapPreset};
    /// let builder = WebMapBuilder::new()
    ///     .set_basemap(BasemapPreset::Streets);
    /// ```
    pub fn set_basemap(mut self, preset: BasemapPreset) -> Self {
        self.basemap = Self::create_basemap_config(preset);
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
    /// # use arcgis_sharing_rs::models::WebMapBuilder;
    /// let builder = WebMapBuilder::new()
    ///     .set_extent(-109.5, 41.0, -109.0, 41.5, 4326);
    /// ```
    pub fn set_extent(mut self, xmin: f64, ymin: f64, xmax: f64, ymax: f64, wkid: i64) -> Self {
        self.initial_state = Some(InitialState {
            viewpoint: Viewpoint {
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
            },
        });
        self
    }

    /// Build the final web map JSON structure
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::models::{WebMapBuilder, BasemapPreset};
    /// let web_map = WebMapBuilder::new()
    ///     .add_feature_layer("https://services.arcgis.com/.../FeatureServer/0", "My Layer")
    ///     .set_basemap(BasemapPreset::Topographic)
    ///     .build();
    /// ```
    pub fn build(self) -> WebMapJson {
        WebMapJson {
            operational_layers: self.operational_layers,
            base_map: self.basemap,
            authoring_app: self.authoring_app,
            authoring_app_version: self.authoring_app_version,
            initial_state: self.initial_state.unwrap_or_else(|| InitialState {
                viewpoint: Viewpoint {
                    target_geometry: TargetGeometry {
                        spatial_reference: SpatialReference {
                            latest_wkid: 3857,
                            wkid: 102100,
                        },
                        xmin: -20037508.342789244,
                        ymin: -20037508.342789244,
                        xmax: 20037508.342789244,
                        ymax: 20037508.342789244,
                    },
                },
            }),
            spatial_reference: self.spatial_reference,
            time_zone: self.time_zone,
            version: self.version,
        }
    }

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
            .set_basemap(BasemapPreset::Topographic)
            .build();

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
            .add_popup_field_with_format("value", "Value", true, true, 2)
            .build();

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
            let web_map = WebMapBuilder::new().set_basemap(preset).build();
            assert_eq!(web_map.base_map.title, expected_title);
        }
    }

    #[test]
    fn test_web_map_builder_extent() {
        let web_map = WebMapBuilder::new()
            .set_extent(-109.5, 41.0, -109.0, 41.5, 4326)
            .build();

        let viewpoint = &web_map.initial_state.viewpoint;
        let geom = &viewpoint.target_geometry;
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
            .set_basemap(BasemapPreset::Topographic)
            .build();

        // Verify it can be serialized to JSON
        let json = serde_json::to_string(&web_map).unwrap();
        assert!(json.contains("operationalLayers"));
        assert!(json.contains("baseMap"));
        assert!(json.contains("Test Layer"));
        assert!(json.contains("Topographic"));
    }
}
