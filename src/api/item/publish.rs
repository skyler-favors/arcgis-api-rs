use crate::{
    api::ItemHandler, builders::publish::PublishParametersBuilder, error::Result, models::*,
};
use reqwest::multipart::Form;
use serde::Serialize;
use snafu::ResultExt;
use url::form_urlencoded;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishItemBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ItemHandler<'a>,

    itemid: String,
    filetype: String,
    publish_parameters: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    build_initial_cache: Option<bool>,
}

impl<'a, 'r> PublishItemBuilder<'a, 'r> {
    pub fn new(handler: &'r ItemHandler<'a>) -> Self {
        Self {
            handler,
            itemid: handler.id.clone(),
            filetype: String::new(),
            publish_parameters: String::new(),
            output_type: None,
            build_initial_cache: None,
        }
    }

    pub fn csv_with_parameters(mut self, builder: PublishParametersBuilder) -> Self {
        self.filetype = "csv".to_string();
        self.publish_parameters = serde_json::to_string(&builder.build()).unwrap();
        self
    }

    /// Set the file type being published (e.g., "csv", "shapefile", "geojson")
    pub fn set_file_type(mut self, filetype: impl Into<String>) -> Self {
        self.filetype = filetype.into();
        self
    }

    /// Set the publish parameters as a JSON object
    pub fn set_publish_parameters(mut self, params: serde_json::Value) -> Self {
        self.publish_parameters = serde_json::to_string(&params).unwrap();
        self
    }

    /// Set the output service type (e.g., "Tiles", "3DTilesService")
    pub fn set_output_type(mut self, output_type: impl Into<String>) -> Self {
        self.output_type = Some(output_type.into());
        self
    }

    /// Enable building initial cache for tiled services
    pub fn set_build_initial_cache(mut self, build: bool) -> Self {
        self.build_initial_cache = Some(build);
        self
    }

    // TODO: I may be able to convert this to a re-usable trait
    fn to_multipart(&self) -> Result<Form> {
        let mut form = Form::new();

        // Serialize all fields (except handler which has #[serde(skip)])
        // The file field will be serialized as a string, which we'll exclude below
        let serialized =
            serde_urlencoded::to_string(self).context(crate::error::SerdeUrlEncodedSnafu)?;

        // Parse back as key-value pairs and add each as text to the form
        for (key, value) in form_urlencoded::parse(serialized.as_bytes()) {
            form = form.text(key.into_owned(), value.into_owned());
        }

        Ok(form)
    }

    pub async fn send(&self) -> Result<PublishItemResponse> {
        let url = self
            .handler
            .client
            .portal
            .join(&format!(
                "sharing/rest/content/users/{}/publish",
                self.handler.username
            ))
            .context(crate::error::UrlParseSnafu)?;

        let form = self.to_multipart()?;

        // POST with params as the body (not query parameters)
        self.handler.client.post_multipart(url, form).await
    }
}
