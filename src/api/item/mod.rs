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

pub struct ItemHandler<'a> {
    client: &'a ArcGISSharingClient,
    username: String,
    id: String,
}

impl<'a> ItemHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, username: String, id: String) -> Self {
        Self {
            client,
            username,
            id,
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

        let response: ItemInfoResult = self.client.get(url, None::<&()>).await?;
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
