mod data;
mod publish;
mod update;

use snafu::ResultExt;

use crate::{
    api::item::{data::ItemDataBuilder, publish::PublishItemBuilder, update::UpdateItemBuilder},
    error::{Result, UrlParseSnafu},
    models::{Item, ItemInfoResult},
    ArcGISSharingClient,
};
use secrecy::SecretString;

pub struct ItemHandler<'a> {
    pub(crate) client: &'a ArcGISSharingClient,
    pub(crate) username: String,
    pub(crate) id: String,
    pub(crate) token: Option<SecretString>,
}

impl<'a> ItemHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, username: String, id: String) -> Self {
        Self {
            client,
            username,
            id,
            token: None,
        }
    }

    pub(crate) fn new_with_token(
        client: &'a ArcGISSharingClient,
        username: String,
        id: String,
        token: SecretString,
    ) -> Self {
        Self {
            client,
            username,
            id,
            token: Some(token),
        }
    }

    pub async fn info(&self) -> Result<Item> {
        let url = self
            .client
            .portal
            .join(&format!(
                "sharing/rest/content/users/{}/items/{}",
                self.username, self.id
            ))
            .context(UrlParseSnafu)?;

        let response: ItemInfoResult = self
            .client
            .get_with_token(url, None::<&()>, self.token.as_ref())
            .await?;
        Ok(response.item)
    }

    pub fn data(&self) -> ItemDataBuilder<'_, '_> {
        ItemDataBuilder::new(self)
    }

    pub fn update(&self) -> UpdateItemBuilder<'_, '_> {
        UpdateItemBuilder::new(self)
    }

    pub fn publish(&self) -> PublishItemBuilder<'_, '_> {
        PublishItemBuilder::new(self)
    }
}
