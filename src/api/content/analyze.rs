use crate::error::UrlParseSnafu;
use crate::{api::ContentHandler, error::Result, models::*};
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use snafu::ResultExt;
use url::form_urlencoded;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ContentHandler<'a>,

    // File type being analyzed (e.g., "csv", "shapefile", "excel", "gpkg")
    #[serde(skip_serializing_if = "Option::is_none")]
    filetype: Option<String>,

    // Content source (mutually exclusive in practice)
    #[serde(skip_serializing_if = "Option::is_none")]
    itemid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    // Optional parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    source_locale: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    source_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    analyze_parameters: Option<serde_json::Value>,

    // File upload control
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
}

impl<'a, 'r> AnalyzeBuilder<'a, 'r> {
    pub fn new(handler: &'r ContentHandler<'a>) -> Self {
        Self {
            handler,
            filetype: Some("csv".to_string()),
            itemid: None,
            file: None,
            text: None,
            source_locale: None,
            source_url: None,
            analyze_parameters: None,
            filename: None,
        }
    }

    /// Set the file type being analyzed
    pub fn set_filetype(mut self, filetype: impl Into<String>) -> Self {
        self.filetype = Some(filetype.into());
        self
    }

    /// Analyze an existing portal item by ID
    pub fn set_item_id(mut self, id: impl Into<String>) -> Self {
        self.itemid = Some(id.into());
        self
    }

    /// Analyze a file by uploading its content
    pub fn set_file_content(mut self, content: impl Into<String>) -> Self {
        self.file = Some(content.into());
        self
    }

    /// Analyze CSV content passed as text
    pub fn set_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set the source locale for the file
    pub fn set_source_locale(mut self, locale: impl Into<String>) -> Self {
        self.source_locale = Some(locale.into());
        self
    }

    /// Set the source URL for the file
    pub fn set_source_url(mut self, url: impl Into<String>) -> Self {
        self.source_url = Some(url.into());
        self
    }

    /// Set custom analyze parameters
    pub fn set_analyze_parameters(mut self, params: serde_json::Value) -> Self {
        self.analyze_parameters = Some(params);
        self
    }

    /// Set the filename for file uploads
    pub fn set_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    /// Returns true if multipart encoding is needed (when file content is present)
    fn needs_multipart(&self) -> bool {
        self.file.is_some()
    }

    /// Convert builder fields to multipart form data
    fn to_multipart(&self) -> Result<Form> {
        let mut form = Form::new();

        // Serialize all fields (except handler and file which need special handling)
        let serialized =
            serde_urlencoded::to_string(self).context(crate::error::SerdeUrlEncodedSnafu)?;

        // Parse back as key-value pairs and add each as text to the form
        for (key, value) in form_urlencoded::parse(serialized.as_bytes()) {
            // Skip the file field - it needs special multipart handling
            if key == "file" {
                continue;
            }
            form = form.text(key.into_owned(), value.into_owned());
        }

        // Handle file upload separately with proper multipart encoding
        if let Some(file_content) = &self.file {
            let filename = self
                .filename
                .clone()
                .unwrap_or_else(|| format!("{}.csv", Uuid::new_v4()));

            let part = Part::bytes(file_content.as_bytes().to_vec())
                .file_name(filename)
                .mime_str("text/csv")
                .context(crate::error::ReqwestSnafu)?;

            form = form.part("file", part);
        }

        Ok(form)
    }

    pub async fn send(&self) -> Result<AnalyzeResponse> {
        let url = self
            .handler
            .client
            .portal
            .join("sharing/rest/content/features/analyze")
            .context(UrlParseSnafu)?;

        if self.needs_multipart() {
            // Use multipart encoding for file uploads
            let form = self.to_multipart()?;
            self.handler.client.post_multipart(url.as_str(), form).await
        } else {
            // Use standard form encoding for non-file requests
            self.handler.client.post(url, Some(self), None).await
        }
    }
}
