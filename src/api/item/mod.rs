mod add_resources;
mod data;
mod delete;
mod get_resource;
mod publish;
mod resources;
mod update;

use std::sync::OnceLock;

use snafu::ResultExt;

use crate::{
    api::item::{
        add_resources::AddResourcesBuilder, data::ItemDataBuilder, delete::DeleteItemBuilder,
        get_resource::GetResourceBuilder, publish::PublishItemBuilder, resources::ResourcesBuilder,
        update::UpdateItemBuilder,
    },
    error::{Result, UrlParseSnafu},
    models::Item,
    ArcGISSharingClient,
};

/// Handler for item-scoped ArcGIS content operations.
///
/// Item id alone is sufficient for read paths (`info`, `data`, `resources`). Mutating
/// operations (`update`, `delete`, `publish`, `add_resources`) resolve the item owner
/// via `GET /content/items/{id}` on first use (or reuse the owner cached by a prior
/// `info()` call).
pub struct ItemHandler<'a> {
    pub(crate) client: &'a ArcGISSharingClient,
    pub(crate) id: String,
    owner: OnceLock<String>,
}

impl<'a> ItemHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, id: String) -> Self {
        Self {
            client,
            id,
            owner: OnceLock::new(),
        }
    }

    async fn fetch_item(&self) -> Result<Item> {
        let url = self
            .client
            .portal
            .join(&format!("sharing/rest/content/items/{}", self.id))
            .context(UrlParseSnafu)?;

        self.client.get(url, None::<&()>).await
    }

    pub(crate) async fn ensure_owner(&self) -> Result<&str> {
        if let Some(owner) = self.owner.get() {
            return Ok(owner.as_str());
        }

        let item = self.fetch_item().await?;
        let _ = self.owner.set(item.owner);
        Ok(self.owner.get().expect("owner set above").as_str())
    }

    pub async fn info(&self) -> Result<Item> {
        let item = self.fetch_item().await?;
        let _ = self.owner.set(item.owner.clone());
        Ok(item)
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

    pub fn delete(&self) -> DeleteItemBuilder<'_, '_> {
        DeleteItemBuilder::new(self)
    }

    pub fn resources(&self) -> ResourcesBuilder<'_, '_> {
        ResourcesBuilder::new(self)
    }

    pub fn add_resources(&self) -> AddResourcesBuilder<'_, '_> {
        AddResourcesBuilder::new(self)
    }

    pub fn get_resource(&self) -> GetResourceBuilder<'_, '_> {
        GetResourceBuilder::new(self)
    }
}
