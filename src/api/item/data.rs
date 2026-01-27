use crate::{api::ItemHandler, error::Result, from_response::FromResponse};
use serde::Serialize;
use snafu::ResultExt;

#[derive(Serialize)]
pub struct ItemDataBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ItemHandler<'a>,
}

impl<'a, 'r> ItemDataBuilder<'a, 'r> {
    pub fn new(handler: &'r ItemHandler<'a>) -> Self {
        Self { handler }
    }

    pub async fn send<T: FromResponse>(&self) -> Result<T> {
        let url = self
            .handler
            .client
            .portal
            .join(&format!(
                "sharing/rest/content/items/{}/data",
                self.handler.id
            ))
            .context(crate::error::UrlParseSnafu)?;

        self.handler.client.get(url, None::<&()>).await
    }
}
