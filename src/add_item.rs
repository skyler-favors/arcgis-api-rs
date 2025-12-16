use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_urlencoded;
use std::path::PathBuf;

use crate::parser::parse_response;

#[derive(Deserialize)]
struct PointCollection {
    points: Vec<[f64; 2]>,
}

pub fn points_json_to_csv(json: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pc: PointCollection = serde_json::from_str(json)?;

    let mut out = String::new();
    out.push_str("Longitude,Latitude\n");

    for [lon, lat] in pc.points {
        out.push_str(&format!("{lon},{lat}\n"));
    }

    Ok(out)
}

pub struct AddItemQuery {
    url: String,
    params: AddItemParams,
}

#[derive(Default)]
pub struct AddItemQueryBuilder {
    url: String,
    params: AddItemParams,
}

/// Parameters for the ArcGIS /content/users/{userName}/addItem endpoint.
///
/// These are serialized as `application/x-www-form-urlencoded` or
/// multipart form fields, depending on whether you send files.
#[derive(Debug, Default, Serialize)]
pub struct AddItemParams {
    // ---- Upload / content source ----
    /// The file to be uploaded (multipart).
    #[serde(skip_serializing_if = "Option::is_none", rename = "file")]
    pub file: Option<String>,

    /// URL where the data file can be downloaded (creates a file item asynchronously).
    /// Docs call this `dataUrl`. :contentReference[oaicite:1]{index=1}
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataUrl")]
    pub data_url: Option<String>,

    /// URL of the item to be submitted (service, web app, etc.).
    #[serde(skip_serializing_if = "Option::is_none", rename = "url")]
    pub url: Option<String>,

    /// JSON string for the item definition (`text` parameter in docs).
    #[serde(skip_serializing_if = "Option::is_none", rename = "text")]
    pub text: Option<String>,

    // ---- Relationships ----
    /// Relationship type between items (comma-delimited string in practice).
    #[serde(skip_serializing_if = "Option::is_none", rename = "relationshipTypes")]
    pub relationship_types: Option<String>,

    /// Origin item ID for a relationship.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originItemId")]
    pub origin_item_id: Option<String>,

    /// Destination item ID for a relationship.
    #[serde(skip_serializing_if = "Option::is_none", rename = "destinationItemId")]
    pub destination_item_id: Option<String>,

    // ---- Multipart upload control ----
    /// If true, upload in multiple parts.
    #[serde(skip_serializing_if = "Option::is_none", rename = "multipart")]
    pub multipart: Option<bool>,

    /// File name when using multipart upload.
    #[serde(skip_serializing_if = "Option::is_none", rename = "filename")]
    pub filename: Option<String>,

    // ---- Service proxy / security ----
    /// JSON object providing rate limiting and referrer checks. :contentReference[oaicite:2]{index=2}
    #[serde(skip_serializing_if = "Option::is_none", rename = "serviceProxyParams")]
    pub service_proxy_params: Option<Value>,

    /// If true, create a proxy item without embedding credentials.
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "createAsServiceProxy"
    )]
    pub create_as_service_proxy: Option<bool>,

    /// Optional explicit item ID for Enterprise (must be 32-char alphanumeric). :contentReference[oaicite:3]{index=3}
    #[serde(skip_serializing_if = "Option::is_none", rename = "itemIdToCreate")]
    pub item_id_to_create: Option<String>,

    // ---- Core item fields ("item parameters") ----
    /// Item title (recommended, but not strictly required).
    #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
    pub title: Option<String>,

    /// Path to thumbnail file to upload (multipart).
    #[serde(skip_serializing_if = "Option::is_none", rename = "thumbnail")]
    pub thumbnail: Option<PathBuf>,

    /// URL to thumbnail image.
    #[serde(skip_serializing_if = "Option::is_none", rename = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,

    /// Metadata XML file (multipart).
    #[serde(skip_serializing_if = "Option::is_none", rename = "metadata")]
    pub metadata: Option<PathBuf>,

    /// Whether metadata is editable on item pages (org-level setting surface). :contentReference[oaicite:4]{index=4}
    #[serde(skip_serializing_if = "Option::is_none", rename = "metadataEditable")]
    pub metadata_editable: Option<bool>,

    /// Metadata style used (e.g., `iso19139`, `dcplus`, etc.).
    #[serde(skip_serializing_if = "Option::is_none", rename = "metadataFormats")]
    pub metadata_formats: Option<String>,

    /// Item type (required). E.g. `"Web Mapping Application"`, `"Feature Service"`. :contentReference[oaicite:5]{index=5}
    #[serde(rename = "type")]
    pub r#type: String,

    /// Additional type keywords (comma-delimited or JSON array).
    #[serde(skip_serializing_if = "Option::is_none", rename = "typeKeywords")]
    pub type_keywords: Option<String>,

    /// Long description text (< 64 KB).
    #[serde(skip_serializing_if = "Option::is_none", rename = "description")]
    pub description: Option<String>,

    /// Tags (comma-delimited string or JSON array).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tags")]
    pub tags: Option<String>,

    /// Short summary / snippet (<= 2048 chars).
    #[serde(skip_serializing_if = "Option::is_none", rename = "snippet")]
    pub snippet: Option<String>,

    /// Classification JSON object (Enterprise 11.4+).
    #[serde(skip_serializing_if = "Option::is_none", rename = "classification")]
    pub classification: Option<Value>,

    /// Bounding box `<xmin>, <ymin>, <xmax>, <ymax>`.
    #[serde(skip_serializing_if = "Option::is_none", rename = "extent")]
    pub extent: Option<String>,

    /// Spatial reference of the item, e.g., `GCS_North_American_1983`.
    #[serde(skip_serializing_if = "Option::is_none", rename = "spatialReference")]
    pub spatial_reference: Option<String>,

    /// Credit / source information.
    #[serde(skip_serializing_if = "Option::is_none", rename = "accessInformation")]
    pub access_information: Option<String>,

    /// License / usage restrictions.
    #[serde(skip_serializing_if = "Option::is_none", rename = "licenseInfo")]
    pub license_info: Option<String>,

    /// Locale (e.g. `en-US`).
    #[serde(skip_serializing_if = "Option::is_none", rename = "culture")]
    pub culture: Option<String>,

    /// Arbitrary properties JSON for marketplace/system metadata.
    #[serde(skip_serializing_if = "Option::is_none", rename = "properties")]
    pub properties: Option<Value>,

    // ---- Marketplace / app-specific fields ----
    #[serde(skip_serializing_if = "Option::is_none", rename = "appCategories")]
    pub app_categories: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "industries")]
    pub industries: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "languages")]
    pub languages: Option<String>,

    /// URL for a larger thumbnail image.
    #[serde(skip_serializing_if = "Option::is_none", rename = "largeThumbnail")]
    pub large_thumbnail: Option<String>,

    /// URL for banner image.
    #[serde(skip_serializing_if = "Option::is_none", rename = "banner")]
    pub banner: Option<String>,

    /// One or more screenshot URLs; ArcGIS accepts multiple fields with the same name.
    /// Represent them as a comma-delimited list or JSON array.
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenshot")]
    pub screenshot: Option<String>,

    /// Listing properties JSON for ArcGIS Marketplace. :contentReference[oaicite:6]{index=6}
    #[serde(skip_serializing_if = "Option::is_none", rename = "listingProperties")]
    pub listing_properties: Option<Value>,

    // ---- Secured service credentials / filters ----
    #[serde(skip_serializing_if = "Option::is_none", rename = "serviceUsername")]
    pub service_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "servicePassword")]
    pub service_password: Option<String>,

    /// JSON filter used for geocode services (sourceCountry/searchExtent/category).
    #[serde(skip_serializing_if = "Option::is_none", rename = "serviceProxyFilter")]
    pub service_proxy_filter: Option<Value>,

    // ---- Category assignments ----
    /// Item categories as JSON array or comma-separated. :contentReference[oaicite:7]{index=7}
    #[serde(skip_serializing_if = "Option::is_none", rename = "categories")]
    pub categories: Option<String>,

    // ---- Async / response format / auth ----
    /// If true, upload is asynchronous (`async` is a Rust keyword).
    #[serde(skip_serializing_if = "Option::is_none", rename = "async")]
    pub async_upload: Option<bool>,

    pub f: String,

    pub token: Option<String>,
}

impl AddItemParams {
    /// Returns true if any file-like fields are present.
    pub fn needs_multipart(&self) -> bool {
        self.file.is_some() || self.thumbnail.is_some() || self.metadata.is_some()
    }

    pub fn to_urlencoded(&self) -> anyhow::Result<String> {
        Ok(serde_urlencoded::to_string(self)?)
    }
}

impl AddItemParams {
    pub fn to_multipart(&self) -> anyhow::Result<Form> {
        let mut form = Form::new();

        macro_rules! add_text {
            ($field:ident, $name:literal) => {
                if let Some(v) = &self.$field {
                    form = form.text($name, v.to_string());
                }
            };
        }

        // ---- Text fields ----
        add_text!(data_url, "dataUrl");
        add_text!(url, "url");
        add_text!(text, "text");
        add_text!(relationship_types, "relationshipTypes");
        add_text!(origin_item_id, "originItemId");
        add_text!(destination_item_id, "destinationItemId");
        add_text!(filename, "filename");
        add_text!(title, "title");
        add_text!(thumbnail_url, "thumbnailUrl");
        add_text!(description, "description");
        add_text!(tags, "tags");
        add_text!(snippet, "snippet");
        add_text!(extent, "extent");
        add_text!(spatial_reference, "spatialReference");
        add_text!(access_information, "accessInformation");
        add_text!(license_info, "licenseInfo");
        add_text!(culture, "culture");
        add_text!(app_categories, "appCategories");
        add_text!(industries, "industries");
        add_text!(languages, "languages");
        add_text!(large_thumbnail, "largeThumbnail");
        add_text!(banner, "banner");
        add_text!(screenshot, "screenshot");
        add_text!(categories, "categories");

        // Required text field
        form = form.text("type", self.r#type.clone());

        // Optional JSON fields
        macro_rules! add_json_value {
            ($field:ident, $name:literal) => {
                if let Some(v) = &self.$field {
                    form = form.text($name, v.to_string());
                }
            };
        }

        add_json_value!(service_proxy_params, "serviceProxyParams");
        add_json_value!(classification, "classification");
        add_json_value!(properties, "properties");
        add_json_value!(listing_properties, "listingProperties");
        add_json_value!(service_proxy_filter, "serviceProxyFilter");

        // Flags
        if let Some(b) = self.multipart {
            form = form.text("multipart", b.to_string());
        }
        if let Some(b) = self.create_as_service_proxy {
            form = form.text("createAsServiceProxy", b.to_string());
        }
        if let Some(b) = self.async_upload {
            form = form.text("async", b.to_string());
        }

        form = form.text("f", "json");

        // TODO: reenable general file upload
        // Currently only supports in memory csv files
        // // ---- File fields ----
        // macro_rules! add_file {
        //     ($field:ident, $name:literal) => {
        //         if let Some(path) = &self.$field {
        //             let bytes = fs::read(path)?;
        //             let filename = path
        //                 .file_name()
        //                 .unwrap_or_default()
        //                 .to_string_lossy()
        //                 .to_string();
        //
        //             let part = Part::bytes(bytes).file_name(filename);
        //             form = form.part($name, part);
        //         }
        //     };
        // }
        //
        // add_file!(file, "file");
        // add_file!(thumbnail, "thumbnail");
        // add_file!(metadata, "metadata");

        if let Some(data) = &self.file {
            let part = Part::bytes(data.as_bytes().to_vec())
                .file_name(uuid::Uuid::new_v4().to_string().replace("-", ""))
                .mime_str("text/csv")
                .unwrap();
            form = form.part("file", part);
        }

        Ok(form)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddItemResponse {
    pub success: bool,
    pub id: String,
    pub folder: Option<String>,
}

impl AddItemQuery {
    pub fn builder(root: impl Into<String>, user_name: impl Into<String>) -> AddItemQueryBuilder {
        AddItemQueryBuilder::new(root, user_name)
    }

    pub async fn send(&self, client: &Client) -> anyhow::Result<AddItemResponse> {
        let response = if self.params.needs_multipart() {
            // ---- Multipart upload ----
            let form = self.params.to_multipart()?;
            client.post(&self.url).multipart(form).send().await?
        } else {
            // ---- URL-encoded form ----
            let body = self.params.to_urlencoded()?;
            client
                .post(&self.url)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body)
                .send()
                .await?
        };

        let body = parse_response::<AddItemResponse>(response).await?;

        Ok(body)
    }
}

impl AddItemQueryBuilder {
    pub fn new(root: impl Into<String>, user_name: impl Into<String>) -> Self {
        // https://[root]/content/users/[userName]/addItem

        let url = format!("{}/content/users/{}/addItem", root.into(), user_name.into());
        // TODO: validtate url
        Self {
            url,
            params: AddItemParams {
                f: "json".into(),
                ..Default::default()
            },
        }
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.params.file = Some(file.into());
        self
    }

    pub fn set_type(mut self, r#type: impl Into<String>) -> Self {
        self.params.r#type = r#type.into();
        self
    }

    pub fn data_url(mut self, data_url: impl Into<String>) -> Self {
        self.params.data_url = Some(data_url.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.params.url = Some(url.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.params.text = Some(text.into());
        self
    }

    pub fn relationship_types(mut self, relationship_types: impl Into<String>) -> Self {
        self.params.relationship_types = Some(relationship_types.into());
        self
    }

    pub fn origin_item_id(mut self, origin_item_id: impl Into<String>) -> Self {
        self.params.origin_item_id = Some(origin_item_id.into());
        self
    }

    pub fn destination_item_id(mut self, destination_item_id: impl Into<String>) -> Self {
        self.params.destination_item_id = Some(destination_item_id.into());
        self
    }

    pub fn multipart(mut self, multipart: bool) -> Self {
        self.params.multipart = Some(multipart);
        self
    }

    pub fn filename(mut self, filename: impl Into<String>) -> Self {
        self.params.filename = Some(filename.into());
        self
    }

    pub fn service_proxy_params(mut self, service_proxy_params: impl Into<Value>) -> Self {
        self.params.service_proxy_params = Some(service_proxy_params.into());
        self
    }

    pub fn create_as_service_proxy(mut self, create_as_service_proxy: bool) -> Self {
        self.params.create_as_service_proxy = Some(create_as_service_proxy);
        self
    }

    pub fn item_id_to_create(mut self, item_id_to_create: impl Into<String>) -> Self {
        self.params.item_id_to_create = Some(item_id_to_create.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.params.title = Some(title.into());
        self
    }

    pub fn thumbnail(mut self, thumbnail: impl Into<PathBuf>) -> Self {
        self.params.thumbnail = Some(thumbnail.into());
        self
    }

    pub fn thumbnail_url(mut self, thumbnail_url: impl Into<String>) -> Self {
        self.params.thumbnail_url = Some(thumbnail_url.into());
        self
    }

    pub fn metadata(mut self, metadata: impl Into<PathBuf>) -> Self {
        self.params.metadata = Some(metadata.into());
        self
    }

    pub fn metadata_editable(mut self, metadata_editable: bool) -> Self {
        self.params.metadata_editable = Some(metadata_editable);
        self
    }

    pub fn service_username(mut self, service_username: impl Into<String>) -> Self {
        self.params.service_username = Some(service_username.into());
        self
    }

    pub fn service_password(mut self, service_password: impl Into<String>) -> Self {
        self.params.service_password = Some(service_password.into());
        self
    }

    pub fn service_proxy_filter(mut self, service_proxy_filter: impl Into<Value>) -> Self {
        self.params.service_proxy_filter = Some(service_proxy_filter.into());
        self
    }

    pub fn categories(mut self, categories: impl Into<String>) -> Self {
        self.params.categories = Some(categories.into());
        self
    }

    pub fn async_upload(mut self, async_upload: bool) -> Self {
        self.params.async_upload = Some(async_upload);
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.params.token = Some(token.into());
        self
    }

    pub fn build(self) -> AddItemQuery {
        let url = if let Some(token) = &self.params.token {
            format!("{}?token={}", self.url, token)
        } else {
            self.url
        };

        AddItemQuery {
            url,
            params: self.params,
        }
    }
}
