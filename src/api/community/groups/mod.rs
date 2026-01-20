mod delete;
mod search;

use crate::{api::community::groups::delete::DeleteGroupsBuilder, ArcGISSharingClient};

pub use search::*;

pub struct GroupsHandler<'a> {
    client: &'a ArcGISSharingClient,
    id: String,
}

impl<'a> GroupsHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, id: impl Into<String>) -> Self {
        Self {
            client,
            id: id.into(),
        }
    }

    pub fn delete(&self) -> DeleteGroupsBuilder<'_, '_> {
        DeleteGroupsBuilder::new(self)
    }
}
