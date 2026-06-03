use crate::{api::ItemHandler, error::Result, models::AddResourcesResponse};
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use snafu::ResultExt;
use url::form_urlencoded;

#[derive(Serialize)]
pub struct AddResourcesBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ItemHandler<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,

    #[serde(rename = "resourcesPrefix", skip_serializing_if = "Option::is_none")]
    resources_prefix: Option<String>,

    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    archive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<String>,
}

impl<'a, 'r> AddResourcesBuilder<'a, 'r> {
    pub fn new(handler: &'r ItemHandler<'a>) -> Self {
        Self {
            handler,
            file: None,
            resources_prefix: None,
            file_name: None,
            text: None,
            archive: None,
            access: None,
            properties: None,
        }
    }

    pub fn file(mut self, content: impl Into<String>) -> Self {
        self.file = Some(content.into());
        self
    }

    pub fn resources_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.resources_prefix = Some(prefix.into());
        self
    }

    pub fn file_name(mut self, name: impl Into<String>) -> Self {
        self.file_name = Some(name.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn archive(mut self, value: bool) -> Self {
        self.archive = Some(value);
        self
    }

    pub fn access(mut self, value: impl Into<String>) -> Self {
        self.access = Some(value.into());
        self
    }

    pub fn properties(mut self, value: impl Into<String>) -> Self {
        self.properties = Some(value.into());
        self
    }

    fn needs_multipart(&self) -> bool {
        self.file.is_some()
    }

    fn to_multipart(&self) -> Result<Form> {
        let mut form = Form::new();

        let serialized =
            serde_urlencoded::to_string(self).context(crate::error::SerdeUrlEncodedSnafu)?;

        for (key, value) in form_urlencoded::parse(serialized.as_bytes()) {
            if key == "file" {
                continue;
            }
            form = form.text(key.into_owned(), value.into_owned());
        }

        if let Some(file_content) = &self.file {
            let filename = self
                .file_name
                .clone()
                .unwrap_or_else(|| "resource".to_string());

            let part = Part::bytes(file_content.as_bytes().to_vec())
                .file_name(filename)
                .mime_str("application/octet-stream")
                .context(crate::error::ReqwestSnafu)?;

            form = form.part("file", part);
        }

        Ok(form)
    }

    pub async fn send(&self) -> Result<AddResourcesResponse> {
        let owner = self.handler.ensure_owner().await?;
        let url = self
            .handler
            .client
            .portal
            .join(&format!(
                "sharing/rest/content/users/{}/items/{}/addResources",
                owner, self.handler.id
            ))
            .context(crate::error::UrlParseSnafu)?;

        if self.needs_multipart() {
            let form = self.to_multipart()?;
            self.handler.client.post_multipart(url.as_str(), form).await
        } else {
            self.handler.client.post(url, Some(self), None).await
        }
    }
}
