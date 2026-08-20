mod add_item;
mod analyze;
mod create_service;

use crate::{
    api::content::{
        add_item::AddItemBuilder, analyze::AnalyzeBuilder, create_service::CreateServiceBuilder,
    },
    ArcGISSharingClient,
};

pub struct ContentHandler<'a> {
    pub(crate) client: &'a ArcGISSharingClient,
    pub(crate) username: String,
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

    pub fn create_service(
        &self,
        create_parameters: serde_json::Value,
    ) -> CreateServiceBuilder<'_, '_> {
        CreateServiceBuilder::new(self, create_parameters)
    }
}
