mod add_item;
mod analyze;

use crate::{
    api::content::{add_item::AddItemBuilder, analyze::AnalyzeBuilder},
    ArcGISSharingClient,
};
use secrecy::SecretString;

pub struct ContentHandler<'a> {
    pub(crate) client: &'a ArcGISSharingClient,
    pub(crate) username: String,
    pub(crate) token: Option<SecretString>,
}

impl<'a> ContentHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, username: String) -> Self {
        Self { client, username, token: None }
    }

    pub(crate) fn new_with_token(
        client: &'a ArcGISSharingClient,
        username: String,
        token: SecretString,
    ) -> Self {
        Self { client, username, token: Some(token) }
    }

    pub fn add_item(&self) -> AddItemBuilder<'_, '_> {
        AddItemBuilder::new(self)
    }

    pub fn analyze(&self) -> AnalyzeBuilder<'_, '_> {
        AnalyzeBuilder::new(self)
    }
}
