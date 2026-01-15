use serde::Serialize;

use crate::api::FeatureServiceHandler;
use crate::{error::Result, models::*};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureServiceQueryBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r FeatureServiceHandler<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_count_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    result_offset: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    out_fields: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_geometry: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r#where: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    result_record_count: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_distinct_values: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_ids_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_extent_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_z: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_m: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_true_curves: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_centroid: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    geometry: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    geometry_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spatial_rel: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    in_sr: Option<String>,
}

impl<'a, 'r> FeatureServiceQueryBuilder<'a, 'r> {
    pub fn new(handler: &'r FeatureServiceHandler<'a>) -> Self {
        Self {
            handler,
            return_count_only: Some(false),
            result_offset: Some(0),
            out_fields: Some("*".to_string()),
            return_geometry: Some(false),
            r#where: Some("1=1".to_string()),
            return_distinct_values: Some(false),
            return_ids_only: Some(false),
            return_extent_only: Some(false),
            return_z: Some(false),
            return_m: Some(false),
            return_true_curves: Some(false),
            return_centroid: Some(false),
            result_record_count: Some(2000),
            geometry: None,
            geometry_type: None,
            spatial_rel: None,
            in_sr: None,
        }
    }

    pub fn set_count_only(mut self, count_only: bool) -> Self {
        self.return_count_only = Some(count_only);
        self
    }

    pub fn set_offset(mut self, offset: i32) -> Self {
        self.result_offset = Some(offset);
        self
    }

    pub fn set_out_fields(mut self, out_fields: impl Into<String>) -> Self {
        self.out_fields = Some(out_fields.into());
        self
    }

    pub fn set_return_geometry(mut self, return_geometry: bool) -> Self {
        self.return_geometry = Some(return_geometry);
        self
    }

    pub fn set_where(mut self, where_clause: impl Into<String>) -> Self {
        self.r#where = Some(where_clause.into());
        self
    }

    pub fn set_result_record_count(mut self, count: i32) -> Self {
        self.result_record_count = Some(count);
        self
    }

    pub fn set_geometry(mut self, geometry: impl Into<String>) -> Self {
        self.geometry = Some(geometry.into());
        self
    }

    pub fn set_geometry_type(mut self, geometry_type: GeometryType) -> Self {
        self.geometry_type = Some(geometry_type.to_esri_string());
        self
    }

    pub fn set_spatial_relationship(mut self, spatial_rel: SpatialRelationship) -> Self {
        self.spatial_rel = Some(spatial_rel.to_esri_string());
        self
    }

    pub fn set_spatial_reference(mut self, sr: u32) -> Self {
        self.in_sr = Some(sr.to_string());
        self
    }

    pub async fn send(&self) -> Result<FeatureServiceQueryResponse> {
        let url = format!("{}/query", self.handler.url);
        self.handler.client.get(url, Some(self)).await
    }
}
