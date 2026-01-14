use serde::Serialize;
use snafu::ResultExt;

use crate::{api::GroupsHandler, error::UrlParseSnafu};
use crate::{error::Result, models::*};

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteGroupsBuilder<'a, 'r> {
    #[serde(skip)]
    handler: Option<&'r GroupsHandler<'a>>,
}

impl<'a, 'r> DeleteGroupsBuilder<'a, 'r> {
    pub fn new(handler: &'r GroupsHandler<'a>) -> Self {
        Self {
            handler: Some(handler),
        }
    }

    pub async fn send(&self) -> Result<DeleteGroupsResponse> {
        let handler = self.handler.as_ref().unwrap();

        let url = handler
            .client
            .portal
            .join(&format!(
                "sharing/rest/community/groups/{}/delete",
                handler.id
            ))
            .context(UrlParseSnafu)?;

        handler.client.post(url, None::<&()>, None).await
    }
}
