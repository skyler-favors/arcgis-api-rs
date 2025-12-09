use log::info;
use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::Value;

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
    pub fn to_esri_string(&self) -> &'static str {
        match self {
            GeometryType::Point => "esriGeometryPoint",
            GeometryType::Polyline => "esriGeometryPolyline",
            GeometryType::Polygon => "esriGeometryPolygon",
            GeometryType::Envelope => "esriGeometryEnvelope",
            GeometryType::Multipoint => "esriGeometryMultipoint",
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
    pub geometry: Value,
    pub geometry_type: GeometryType,
    pub spatial_rel: SpatialRelationship,
    pub in_sr: Option<u32>, // Input spatial reference
}

impl QueryGeometry {
    pub fn new(
        geometry: Value,
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

#[derive(Deserialize)]
pub struct EsriCountResponse {
    pub count: i32,
}

#[derive(Deserialize, Debug)]
pub struct EsriQueryResponse {
    pub features: Vec<EsriFeature>,
}

#[derive(Deserialize, Debug)]
pub struct EsriFeature {
    pub attributes: Value,
    pub geometry: Option<Value>,
}

pub struct FeatureLayerQuery {
    count_only: String,
    offset: String,
    out_fields: String,
    return_geometry: String,
    where_clause: String,
    geometry: Option<String>,
    geometry_type: Option<String>,
    spatial_rel: Option<String>,
    in_sr: Option<String>,
}

impl FeatureLayerQuery {
    pub fn builder() -> FeatureLayerQueryBuilder {
        FeatureLayerQueryBuilder::new()
    }

    pub async fn send(&self, client: &Client, url: &str) -> Result<Response, reqwest::Error> {
        let url = format!("{}/query", url);

        let mut query: Vec<(&str, &str)> = vec![
            ("returnCountOnly", &self.count_only),
            ("resultOffset", &self.offset),
            ("where", &self.where_clause),
            ("units", "esriSRUnit_Foot"),
            ("outFields", &self.out_fields),
            ("returnGeometry", &self.return_geometry),
            ("returnDistinctValues", "false"),
            ("returnIdsOnly", "false"),
            ("resultRecordCount", "2000"),
            ("returnExtentOnly", "false"),
            ("returnZ", "false"),
            ("returnM", "false"),
            ("returnTrueCurves", "false"),
            ("returnCentroid", "false"),
            ("returnEnvelope", "false"),
            ("timeReferenceUnknownClient", "false"),
            ("lodType", "geohash"),
            ("cacheHint", "false"),
            ("f", "json"),
        ];

        // TODO: Set defaults
        // Add geometry parameters if present
        if let Some(ref geometry) = self.geometry {
            query.push(("geometry", geometry));
        }
        if let Some(ref geometry_type) = self.geometry_type {
            query.push(("geometryType", geometry_type));
        }
        if let Some(ref spatial_rel) = self.spatial_rel {
            query.push(("spatialRel", spatial_rel));
        }
        if let Some(ref in_sr) = self.in_sr {
            query.push(("inSR", in_sr));
        }

        info!(
            "[FeatureLayerQuery] sending query: {} : offset {} : countOnly {} : geometry {}",
            url,
            &self.offset,
            &self.count_only,
            if self.geometry.is_some() {
                "present"
            } else {
                "none"
            }
        );
        client.get(url).query(&query).send().await
    }
}

//#[derive(Default)]
pub struct FeatureLayerQueryBuilder {
    count_only: String,
    offset: String,
    out_fields: String,
    return_geometry: bool,
    where_clause: String,
    //geometry: Option<QueryGeometry>,
    geometry: Option<String>,
    geometry_type: Option<GeometryType>,
    spatial_rel: Option<SpatialRelationship>,
    in_sr: Option<u32>,
}

impl FeatureLayerQueryBuilder {
    pub fn new() -> FeatureLayerQueryBuilder {
        FeatureLayerQueryBuilder {
            count_only: "false".to_string(),
            offset: "0".to_string(),
            out_fields: "*".to_string(),
            return_geometry: false,
            where_clause: "1=1".to_string(),
            geometry: None,
            geometry_type: None,
            spatial_rel: None,
            in_sr: None,
        }
    }

    pub fn set_count_only(mut self, count_only: bool) -> FeatureLayerQueryBuilder {
        self.count_only = format!("{}", count_only);
        self
    }

    pub fn set_offset(mut self, offset: i32) -> FeatureLayerQueryBuilder {
        self.offset = format!("{}", offset);
        self
    }

    pub fn set_out_fields_vec(mut self, out_fields: Vec<String>) -> FeatureLayerQueryBuilder {
        let o: &str = if out_fields.len() == 0 {
            "*"
        } else {
            &out_fields.join(",")
        };

        self.out_fields = o.to_string();
        self
    }

    pub fn set_out_fields(mut self, out_fields: impl Into<String>) -> FeatureLayerQueryBuilder {
        self.out_fields = out_fields.into();
        self
    }

    pub fn set_return_geometry(mut self, return_geometry: bool) -> FeatureLayerQueryBuilder {
        self.return_geometry = return_geometry;
        self
    }

    pub fn set_where(mut self, where_clause: impl Into<String>) -> FeatureLayerQueryBuilder {
        self.where_clause = where_clause.into();
        self
    }

    pub fn set_geometry(mut self, geometry: String) -> FeatureLayerQueryBuilder {
        self.geometry = Some(geometry);
        self
    }

    // /// Set geometry for spatial queries
    // pub fn set_geometry(mut self, geometry: QueryGeometry) -> FeatureLayerQueryBuilder {
    //     self.geometry = Some(geometry);
    //     self
    // }
    //
    // /// Convenience method for envelope/bounding box queries
    // pub fn set_envelope(
    //     mut self,
    //     xmin: f64,
    //     ymin: f64,
    //     xmax: f64,
    //     ymax: f64,
    //     spatial_ref: Option<u32>,
    // ) -> FeatureLayerQueryBuilder {
    //     let mut envelope =
    //         QueryGeometry::envelope(xmin, ymin, xmax, ymax, SpatialRelationship::Intersects);
    //     if let Some(sr) = spatial_ref {
    //         envelope = envelope.with_spatial_reference(sr);
    //     }
    //     self.geometry = Some(envelope);
    //     self
    // }
    //
    // /// Convenience method for envelope queries with custom spatial relationship
    // pub fn set_envelope_with_relationship(
    //     mut self,
    //     xmin: f64,
    //     ymin: f64,
    //     xmax: f64,
    //     ymax: f64,
    //     spatial_rel: SpatialRelationship,
    //     spatial_ref: Option<u32>,
    // ) -> FeatureLayerQueryBuilder {
    //     let mut envelope = QueryGeometry::envelope(xmin, ymin, xmax, ymax, spatial_rel);
    //     if let Some(sr) = spatial_ref {
    //         envelope = envelope.with_spatial_reference(sr);
    //     }
    //     self.geometry = Some(envelope);
    //     self
    // }
    //
    // /// Convenience method for point-based queries
    // pub fn set_point(
    //     mut self,
    //     x: f64,
    //     y: f64,
    //     spatial_ref: Option<u32>,
    // ) -> FeatureLayerQueryBuilder {
    //     let mut point = QueryGeometry::point(x, y, SpatialRelationship::Intersects);
    //     if let Some(sr) = spatial_ref {
    //         point = point.with_spatial_reference(sr);
    //     }
    //     self.geometry = Some(point);
    //     self
    // }
    //
    // /// Convenience method for point queries with custom spatial relationship
    // pub fn set_point_with_relationship(
    //     mut self,
    //     x: f64,
    //     y: f64,
    //     spatial_rel: SpatialRelationship,
    //     spatial_ref: Option<u32>,
    // ) -> FeatureLayerQueryBuilder {
    //     let mut point = QueryGeometry::point(x, y, spatial_rel);
    //     if let Some(sr) = spatial_ref {
    //         point = point.with_spatial_reference(sr);
    //     }
    //     self.geometry = Some(point);
    //     self
    // }
    //
    // /// Convenience method for polygon-based queries
    // pub fn set_polygon(
    //     mut self,
    //     rings: Vec<Vec<[f64; 2]>>,
    //     spatial_rel: SpatialRelationship,
    //     spatial_ref: Option<u32>,
    // ) -> FeatureLayerQueryBuilder {
    //     let mut polygon = QueryGeometry::polygon(rings, spatial_rel);
    //     if let Some(sr) = spatial_ref {
    //         polygon = polygon.with_spatial_reference(sr);
    //     }
    //     self.geometry = Some(polygon);
    //     self
    // }
    //
    /// Set spatial relationship for existing geometry (if any)
    // pub fn set_spatial_relationship(
    //     mut self,
    //     spatial_rel: SpatialRelationship,
    // ) -> FeatureLayerQueryBuilder {
    //     if let Some(mut geom) = self.geometry {
    //         geom.spatial_rel = spatial_rel;
    //         self.geometry = Some(geom);
    //     }
    //     self
    // }

    pub fn set_spatial_reference(mut self, sr: u32) -> FeatureLayerQueryBuilder {
        self.in_sr = Some(sr);
        self
    }

    pub fn set_geometry_type(mut self, geometry_type: GeometryType) -> FeatureLayerQueryBuilder {
        self.geometry_type = Some(geometry_type);
        self
    }

    pub fn set_spatial_relationship(
        mut self,
        spatial_rel: SpatialRelationship,
    ) -> FeatureLayerQueryBuilder {
        self.spatial_rel = Some(spatial_rel);
        self
    }

    pub fn build(self) -> FeatureLayerQuery {
        // TODO: add validation
        // if self.out_fields.trim().is_empty() {
        //     return Err(anyhow::anyhow!("Missing out_fields"));
        // }

        FeatureLayerQuery {
            count_only: self.count_only,
            offset: self.offset,
            out_fields: self.out_fields,
            return_geometry: self.return_geometry.to_string(),
            where_clause: self.where_clause,
            geometry: self.geometry,
            geometry_type: self.geometry_type.map(|t| t.to_esri_string().to_string()),
            spatial_rel: self.spatial_rel.map(|r| r.to_esri_string()),
            in_sr: self.in_sr.map(|sr| sr.to_string()),
        }
    }
}
