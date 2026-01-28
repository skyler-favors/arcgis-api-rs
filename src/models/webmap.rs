use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub feature_effect: Option<Value>,
    pub show_labels: Option<bool>,
    pub effect: Option<Vec<Effect>>,
    pub blend_mode: Option<String>,
    pub style_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerDefinition {
    pub feature_reduction: Option<Value>,
    pub drawing_info: Option<WebMapDrawingInfo>,
    pub definition_expression: Option<Value>,
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
pub struct WebMapSymbol2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub symbol_layers: Vec<SymbolLayer>,
    pub angle_alignment: String,
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
    pub symbol: WebMapSymbol,
    pub label: Option<String>,
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
    pub expression_infos: Option<Vec<Value>>,
    pub field_infos: Vec<FieldInfo>,
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
    pub effect: Option<Vec<Effect2>>,
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
    pub viewpoint: Option<Viewpoint>,
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
