mod add_item;

use crate::{
    api::content::add_item::AddItemBuilder,
    error::{Result, UrlParseSnafu},
    models::FeatureServiceInfo,
    ArcGISSharingClient,
};

pub struct ContentHandler<'a> {
    client: &'a ArcGISSharingClient,
    username: String,
}

impl<'a> ContentHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, username: String) -> Self {
        Self { client, username }
    }

    pub fn add_item(&self) -> AddItemBuilder<'_, '_> {
        AddItemBuilder::new(self)
    }
}
