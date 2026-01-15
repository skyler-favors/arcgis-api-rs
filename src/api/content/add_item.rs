use crate::error::UrlParseSnafu;
use crate::{api::ContentHandler, error::Result, models::*};
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use snafu::ResultExt;
use url::form_urlencoded;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddItemBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ContentHandler<'a>,

    // Content source (mutually exclusive in practice)
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    // Required field
    r#type: String,

    // Core metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    snippet: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_url: Option<String>,

    // Type information
    #[serde(skip_serializing_if = "Option::is_none")]
    type_keywords: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<String>,

    // Spatial information
    #[serde(skip_serializing_if = "Option::is_none")]
    extent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spatial_reference: Option<String>,

    // Credits and licensing
    #[serde(skip_serializing_if = "Option::is_none")]
    access_information: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    license_info: Option<String>,

    // Locale
    #[serde(skip_serializing_if = "Option::is_none")]
    culture: Option<String>,

    // Advanced metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    classification: Option<serde_json::Value>,

    // Service proxy settings
    #[serde(skip_serializing_if = "Option::is_none")]
    service_proxy_params: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    create_as_service_proxy: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    service_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    service_password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    service_proxy_filter: Option<serde_json::Value>,

    // Relationships
    #[serde(skip_serializing_if = "Option::is_none")]
    relationship_types: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    origin_item_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    destination_item_id: Option<String>,

    // Upload control
    #[serde(skip_serializing_if = "Option::is_none")]
    multipart: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "async")]
    async_upload: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_id_to_create: Option<String>,

    // Marketplace/Application fields
    #[serde(skip_serializing_if = "Option::is_none")]
    app_categories: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    industries: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    languages: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    large_thumbnail: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    banner: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    screenshot: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    listing_properties: Option<serde_json::Value>,

    // Metadata settings
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_editable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_formats: Option<String>,
}

impl<'a, 'r> AddItemBuilder<'a, 'r> {
    pub fn new(handler: &'r ContentHandler<'a>) -> Self {
        Self {
            handler,
            file: None,
            url: None,
            data_url: None,
            text: None,
            r#type: String::new(),
            title: None,
            description: None,
            tags: None,
            snippet: None,
            thumbnail_url: None,
            type_keywords: None,
            categories: None,
            extent: None,
            spatial_reference: None,
            access_information: None,
            license_info: None,
            culture: None,
            properties: None,
            classification: None,
            service_proxy_params: None,
            create_as_service_proxy: None,
            service_username: None,
            service_password: None,
            service_proxy_filter: None,
            relationship_types: None,
            origin_item_id: None,
            destination_item_id: None,
            multipart: None,
            filename: None,
            async_upload: None,
            item_id_to_create: None,
            app_categories: None,
            industries: None,
            languages: None,
            large_thumbnail: None,
            banner: None,
            screenshot: None,
            listing_properties: None,
            metadata_editable: None,
            metadata_formats: None,
        }
    }

    // Content source setters
    pub fn file(mut self, content: impl Into<String>) -> Self {
        self.file = Some(content.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn data_url(mut self, data_url: impl Into<String>) -> Self {
        self.data_url = Some(data_url.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    // Required type setter
    pub fn set_type(mut self, item_type: impl Into<String>) -> Self {
        self.r#type = item_type.into();
        self
    }

    // Core metadata setters
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn tags(mut self, tags: impl Into<String>) -> Self {
        self.tags = Some(tags.into());
        self
    }

    pub fn snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }

    pub fn thumbnail_url(mut self, url: impl Into<String>) -> Self {
        self.thumbnail_url = Some(url.into());
        self
    }

    // Type information setters
    pub fn type_keywords(mut self, keywords: impl Into<String>) -> Self {
        self.type_keywords = Some(keywords.into());
        self
    }

    pub fn categories(mut self, categories: impl Into<String>) -> Self {
        self.categories = Some(categories.into());
        self
    }

    // Spatial information setters
    pub fn extent(mut self, extent: impl Into<String>) -> Self {
        self.extent = Some(extent.into());
        self
    }

    pub fn spatial_reference(mut self, sr: impl Into<String>) -> Self {
        self.spatial_reference = Some(sr.into());
        self
    }

    // Credits and licensing setters
    pub fn access_information(mut self, info: impl Into<String>) -> Self {
        self.access_information = Some(info.into());
        self
    }

    pub fn license_info(mut self, info: impl Into<String>) -> Self {
        self.license_info = Some(info.into());
        self
    }

    // Locale setter
    pub fn culture(mut self, culture: impl Into<String>) -> Self {
        self.culture = Some(culture.into());
        self
    }

    // Advanced metadata setters
    pub fn properties(mut self, properties: serde_json::Value) -> Self {
        self.properties = Some(properties);
        self
    }

    pub fn classification(mut self, classification: serde_json::Value) -> Self {
        self.classification = Some(classification);
        self
    }

    // Service proxy setters
    pub fn service_proxy_params(mut self, params: serde_json::Value) -> Self {
        self.service_proxy_params = Some(params);
        self
    }

    pub fn create_as_service_proxy(mut self, value: bool) -> Self {
        self.create_as_service_proxy = Some(value);
        self
    }

    pub fn service_username(mut self, username: impl Into<String>) -> Self {
        self.service_username = Some(username.into());
        self
    }

    pub fn service_password(mut self, password: impl Into<String>) -> Self {
        self.service_password = Some(password.into());
        self
    }

    pub fn service_proxy_filter(mut self, filter: serde_json::Value) -> Self {
        self.service_proxy_filter = Some(filter);
        self
    }

    // Relationship setters
    pub fn relationship_types(mut self, types: impl Into<String>) -> Self {
        self.relationship_types = Some(types.into());
        self
    }

    pub fn origin_item_id(mut self, id: impl Into<String>) -> Self {
        self.origin_item_id = Some(id.into());
        self
    }

    pub fn destination_item_id(mut self, id: impl Into<String>) -> Self {
        self.destination_item_id = Some(id.into());
        self
    }

    // Upload control setters
    pub fn multipart(mut self, value: bool) -> Self {
        self.multipart = Some(value);
        self
    }

    pub fn filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    pub fn async_upload(mut self, value: bool) -> Self {
        self.async_upload = Some(value);
        self
    }

    pub fn item_id_to_create(mut self, id: impl Into<String>) -> Self {
        self.item_id_to_create = Some(id.into());
        self
    }

    // Marketplace/Application setters
    pub fn app_categories(mut self, categories: impl Into<String>) -> Self {
        self.app_categories = Some(categories.into());
        self
    }

    pub fn industries(mut self, industries: impl Into<String>) -> Self {
        self.industries = Some(industries.into());
        self
    }

    pub fn languages(mut self, languages: impl Into<String>) -> Self {
        self.languages = Some(languages.into());
        self
    }

    pub fn large_thumbnail(mut self, url: impl Into<String>) -> Self {
        self.large_thumbnail = Some(url.into());
        self
    }

    pub fn banner(mut self, url: impl Into<String>) -> Self {
        self.banner = Some(url.into());
        self
    }

    pub fn screenshot(mut self, url: impl Into<String>) -> Self {
        self.screenshot = Some(url.into());
        self
    }

    pub fn listing_properties(mut self, properties: serde_json::Value) -> Self {
        self.listing_properties = Some(properties);
        self
    }

    // Metadata setters
    pub fn metadata_editable(mut self, value: bool) -> Self {
        self.metadata_editable = Some(value);
        self
    }

    pub fn metadata_formats(mut self, formats: impl Into<String>) -> Self {
        self.metadata_formats = Some(formats.into());
        self
    }

    /// Returns true if multipart encoding is needed (when file content is present)
    fn needs_multipart(&self) -> bool {
        self.file.is_some()
    }

    /// Convert builder fields to multipart form data
    /// Leverages existing Serialize implementation to automatically handle all fields
    fn to_multipart(&self) -> Result<Form> {
        let mut form = Form::new();

        // Serialize all fields (except handler which has #[serde(skip)])
        // The file field will be serialized as a string, which we'll exclude below
        let serialized = serde_urlencoded::to_string(self)
            .context(crate::error::SerdeUrlEncodedSnafu)?;

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

    pub async fn send(&self) -> Result<AddItemResponse> {
        let url = self
            .handler
            .client
            .portal
            .join(&format!(
                "sharing/rest/content/users/{}/addItem",
                self.handler.username
            ))
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
