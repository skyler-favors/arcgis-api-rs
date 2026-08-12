use serde::Serialize;

use crate::api::serialize_json_string;
use crate::api::FeatureServiceHandler;
use crate::{error::Result, models::*};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureServiceApplyEditsBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r FeatureServiceHandler<'a>,

    // Core edit arrays - serialize as JSON strings
    #[serde(
        serialize_with = "serialize_json_string",
        skip_serializing_if = "Vec::is_empty"
    )]
    adds: Vec<serde_json::Value>,

    #[serde(
        serialize_with = "serialize_json_string",
        skip_serializing_if = "Vec::is_empty"
    )]
    updates: Vec<serde_json::Value>,

    #[serde(
        serialize_with = "serialize_json_string",
        skip_serializing_if = "Vec::is_empty"
    )]
    deletes: Vec<serde_json::Value>,

    // Optional parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    gdb_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rollback_on_failure: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    use_global_ids: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_edit_moment: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_edit_results: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    true_curve_client: Option<bool>,

    #[serde(
        serialize_with = "serialize_json_string",
        skip_serializing_if = "Vec::is_empty"
    )]
    attachments: Vec<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_reference_unknown_client: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    datum_transformation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    edits_upload_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r#async: Option<bool>,
}

impl<'a, 'r> FeatureServiceApplyEditsBuilder<'a, 'r> {
    pub fn new(handler: &'r FeatureServiceHandler<'a>) -> Self {
        Self {
            handler,
            adds: Vec::new(),
            updates: Vec::new(),
            deletes: Vec::new(),
            gdb_version: None,
            rollback_on_failure: Some(true),
            use_global_ids: Some(false),
            return_edit_moment: Some(false),
            return_edit_results: Some(true),
            true_curve_client: Some(true),
            attachments: Vec::new(),
            time_reference_unknown_client: Some(false),
            datum_transformation: None,
            edits_upload_id: None,
            r#async: Some(false),
        }
    }

    /// Set features to add
    pub fn set_adds(mut self, adds: Vec<serde_json::Value>) -> Self {
        self.adds = adds;
        self
    }

    /// Set features to update
    pub fn set_updates(mut self, updates: Vec<serde_json::Value>) -> Self {
        self.updates = updates;
        self
    }

    /// Set features to delete (can be object IDs or features with object IDs)
    pub fn set_deletes(mut self, deletes: Vec<serde_json::Value>) -> Self {
        self.deletes = deletes;
        self
    }

    /// Set features to delete by object IDs
    pub fn set_delete_ids(mut self, object_ids: Vec<i64>) -> Self {
        self.deletes = object_ids
            .into_iter()
            .map(|id| serde_json::json!(id))
            .collect();
        self
    }

    /// Set the geodatabase version to apply edits to (default: empty string = default version)
    pub fn set_gdb_version(mut self, version: impl Into<String>) -> Self {
        self.gdb_version = Some(version.into());
        self
    }

    /// Set whether to rollback all edits if any fail (default: true)
    pub fn set_rollback_on_failure(mut self, rollback: bool) -> Self {
        self.rollback_on_failure = Some(rollback);
        self
    }

    /// Set whether to use global IDs for identification (default: false)
    pub fn set_use_global_ids(mut self, use_global_ids: bool) -> Self {
        self.use_global_ids = Some(use_global_ids);
        self
    }

    /// Set whether to return the edit moment (default: false)
    pub fn set_return_edit_moment(mut self, return_moment: bool) -> Self {
        self.return_edit_moment = Some(return_moment);
        self
    }

    /// Set whether to return edit results (default: true)
    pub fn set_return_edit_results(mut self, return_results: bool) -> Self {
        self.return_edit_results = Some(return_results);
        self
    }

    /// Set true curve client handling (default: true)
    pub fn set_true_curve_client(mut self, true_curve: bool) -> Self {
        self.true_curve_client = Some(true_curve);
        self
    }

    /// Set attachments to add or update
    pub fn set_attachments(mut self, attachments: Vec<serde_json::Value>) -> Self {
        self.attachments = attachments;
        self
    }

    /// Set time reference unknown client flag (default: false)
    pub fn set_time_reference_unknown_client(mut self, unknown: bool) -> Self {
        self.time_reference_unknown_client = Some(unknown);
        self
    }

    /// Set datum transformation for coordinate system conversion
    pub fn set_datum_transformation(mut self, transformation: impl Into<String>) -> Self {
        self.datum_transformation = Some(transformation.into());
        self
    }

    /// Set edits upload ID for large edit operations
    pub fn set_edits_upload_id(mut self, upload_id: impl Into<String>) -> Self {
        self.edits_upload_id = Some(upload_id.into());
        self
    }

    /// Set whether to run asynchronously (default: false)
    pub fn set_async(mut self, async_mode: bool) -> Self {
        self.r#async = Some(async_mode);
        self
    }

    /// Execute the apply edits request
    pub async fn send(&self) -> Result<ApplyEditsResponse> {
        let url = format!("{}/applyEdits", self.handler.url);
        self.handler.client.post(url, Some(self), None).await
    }
}
