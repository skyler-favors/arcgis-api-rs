use serde::Serialize;
use snafu::ResultExt;

use crate::{
    api::item::ItemHandler,
    error::{Result, UrlParseSnafu},
    models::DeleteItemResponse,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteItemBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ItemHandler<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    permanent_delete: Option<bool>,
}

impl<'a, 'r> DeleteItemBuilder<'a, 'r> {
    pub fn new(handler: &'r ItemHandler<'a>) -> Self {
        Self {
            handler,
            force: None,
            permanent_delete: None,
        }
    }

    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }

    pub fn permanent_delete(mut self, permanent_delete: bool) -> Self {
        self.permanent_delete = Some(permanent_delete);
        self
    }

    pub async fn send(&self) -> Result<DeleteItemResponse> {
        let url = self
            .handler
            .client
            .portal
            .join(&format!(
                "sharing/rest/content/users/{}/items/{}/delete",
                self.handler.username, self.handler.id
            ))
            .context(UrlParseSnafu)?;

        self.handler.client.post(url, Some(self), None).await
    }
}
