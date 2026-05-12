use crate::api::ItemHandler;
use crate::error::{ReqwestSnafu, Result};
use serde::Serialize;
use snafu::ResultExt;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ItemHandler<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    w: Option<u32>,
}

impl<'a, 'r> GetResourceBuilder<'a, 'r> {
    pub(crate) fn new(handler: &'a ItemHandler) -> Self {
        Self { handler, w: None }
    }

    pub fn set_width(mut self, w: u32) -> Self {
        self.w = Some(w);
        self
    }

    pub async fn send(&self, filename: &str) -> Result<String> {
        let url = self
            .handler
            .client
            .portal
            .join(&format!(
                "sharing/rest/content/items/{}/resources/{}",
                self.handler.id, filename
            ))
            .context(crate::error::UrlParseSnafu)?;

        self.handler
            .client
            ._get(url)
            .await?
            .text()
            .await
            .context(ReqwestSnafu)
    }
}
