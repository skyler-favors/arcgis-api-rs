use serde::Serialize;
use snafu::ResultExt;

use crate::{error::Result, error::UrlParseSnafu, models::*, ArcGISSharingClient};

#[derive(Serialize)]
pub struct CommunitySelfBuilder<'a> {
    #[serde(skip)]
    client: &'a ArcGISSharingClient,
}

impl<'a> CommunitySelfBuilder<'a> {
    pub fn new(client: &'a ArcGISSharingClient) -> Self {
        Self { client }
    }

    pub async fn send(&self) -> Result<UserSelfResponse> {
        let url = self
            .client
            .portal
            .join("sharing/rest/community/self")
            .context(UrlParseSnafu)?;

        self.client
            .get(url, None::<&()>)
            .await
    }
}
