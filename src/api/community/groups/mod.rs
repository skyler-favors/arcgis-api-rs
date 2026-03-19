mod delete;
mod search;

use crate::{api::community::groups::delete::DeleteGroupsBuilder, ArcGISSharingClient};
use secrecy::SecretString;

pub use search::*;

pub struct GroupsHandler<'a> {
    pub(crate) client: &'a ArcGISSharingClient,
    pub(crate) id: String,
    pub(crate) token: Option<SecretString>,
}

impl<'a> GroupsHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, id: impl Into<String>) -> Self {
        Self {
            client,
            id: id.into(),
            token: None,
        }
    }

    pub(crate) fn new_with_token(
        client: &'a ArcGISSharingClient,
        id: impl Into<String>,
        token: SecretString,
    ) -> Self {
        Self {
            client,
            id: id.into(),
            token: Some(token),
        }
    }

    pub fn delete(&self) -> DeleteGroupsBuilder<'_, '_> {
        DeleteGroupsBuilder::new(self)
    }
}
