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

/// Geometry types supported by ArcGIS REST API
#[derive(Debug, Clone)]
pub enum GeometryType {
    Point,
    Polyline,
    Polygon,
    Envelope,
    Multipoint,
}

impl GeometryType {
    pub fn to_esri_string(&self) -> String {
        match self {
            GeometryType::Point => "esriGeometryPoint".to_string(),
            GeometryType::Polyline => "esriGeometryPolyline".to_string(),
            GeometryType::Polygon => "esriGeometryPolygon".to_string(),
            GeometryType::Envelope => "esriGeometryEnvelope".to_string(),
            GeometryType::Multipoint => "esriGeometryMultipoint".to_string(),
        }
    }
}

/// Spatial relationship operators for geometry queries
#[derive(Debug, Clone)]
pub enum SpatialRelationship {
    Intersects,
    Contains,
    Crosses,
    EnvelopeIntersects,
    IndexIntersects,
    Overlaps,
    Touches,
    Within,
    Relation(String), // Custom DE-9IM relation
}

impl SpatialRelationship {
    pub fn to_esri_string(&self) -> String {
        match self {
            SpatialRelationship::Intersects => "esriSpatialRelIntersects".to_string(),
            SpatialRelationship::Contains => "esriSpatialRelContains".to_string(),
            SpatialRelationship::Crosses => "esriSpatialRelCrosses".to_string(),
            SpatialRelationship::EnvelopeIntersects => {
                "esriSpatialRelEnvelopeIntersects".to_string()
            }
            SpatialRelationship::IndexIntersects => "esriSpatialRelIndexIntersects".to_string(),
            SpatialRelationship::Overlaps => "esriSpatialRelOverlaps".to_string(),
            SpatialRelationship::Touches => "esriSpatialRelTouches".to_string(),
            SpatialRelationship::Within => "esriSpatialRelWithin".to_string(),
            SpatialRelationship::Relation(relation) => {
                format!("esriSpatialRelRelation={}", relation)
            }
        }
    }
}

/// Geometry wrapper for spatial queries
#[derive(Debug, Clone)]
pub struct QueryGeometry {
    pub geometry: serde_json::Value,
    pub geometry_type: GeometryType,
    pub spatial_rel: SpatialRelationship,
    pub in_sr: Option<u32>, // Input spatial reference
}

impl QueryGeometry {
    pub fn new(
        geometry: serde_json::Value,
        geometry_type: GeometryType,
        spatial_rel: SpatialRelationship,
    ) -> Self {
        QueryGeometry {
            geometry,
            geometry_type,
            spatial_rel,
            in_sr: None,
        }
    }

    pub fn with_spatial_reference(mut self, sr: u32) -> Self {
        self.in_sr = Some(sr);
        self
    }

    /// Create an envelope geometry for bounding box queries
    pub fn envelope(
        xmin: f64,
        ymin: f64,
        xmax: f64,
        ymax: f64,
        spatial_rel: SpatialRelationship,
    ) -> Self {
        let envelope = serde_json::json!({
            "xmin": xmin,
            "ymin": ymin,
            "xmax": xmax,
            "ymax": ymax
        });

        QueryGeometry {
            geometry: envelope,
            geometry_type: GeometryType::Envelope,
            spatial_rel,
            in_sr: None,
        }
    }

    /// Create a point geometry for point-based queries
    pub fn point(x: f64, y: f64, spatial_rel: SpatialRelationship) -> Self {
        let point = serde_json::json!({
            "x": x,
            "y": y
        });

        QueryGeometry {
            geometry: point,
            geometry_type: GeometryType::Point,
            spatial_rel,
            in_sr: None,
        }
    }

    /// Create a polygon geometry from coordinate rings
    pub fn polygon(rings: Vec<Vec<[f64; 2]>>, spatial_rel: SpatialRelationship) -> Self {
        let polygon = serde_json::json!({
            "rings": rings
        });

        QueryGeometry {
            geometry: polygon,
            geometry_type: GeometryType::Polygon,
            spatial_rel,
            in_sr: None,
        }
    }
}
