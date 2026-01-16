mod publish;
mod update;

use crate::{
    api::item::{publish::PublishItemBuilder, update::UpdateItemBuilder},
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

    pub fn update(&self) -> UpdateItemBuilder<'_, '_> {
        UpdateItemBuilder::new(self)
    }

    pub fn publish(&self) -> PublishItemBuilder<'_, '_> {
        PublishItemBuilder::new(self)
    }
}
