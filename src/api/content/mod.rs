mod add_item;
mod analyze;

use crate::{
    api::content::{add_item::AddItemBuilder, analyze::AnalyzeBuilder},
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

    pub fn analyze(&self) -> AnalyzeBuilder<'_, '_> {
        AnalyzeBuilder::new(self)
    }
}
