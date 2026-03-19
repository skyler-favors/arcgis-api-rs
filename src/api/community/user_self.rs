use secrecy::SecretString;
use serde::Serialize;
use snafu::ResultExt;

use crate::{error::Result, error::UrlParseSnafu, models::*, ArcGISSharingClient};

#[derive(Serialize)]
pub struct CommunitySelfBuilder<'a> {
    #[serde(skip)]
    client: &'a ArcGISSharingClient,
    #[serde(skip)]
    pub(crate) token: Option<SecretString>,
}

impl<'a> CommunitySelfBuilder<'a> {
    pub fn new(client: &'a ArcGISSharingClient) -> Self {
        Self { client, token: None }
    }

    pub async fn send(&self) -> Result<UserSelfResponse> {
        let url = self
            .client
            .portal
            .join("sharing/rest/community/self")
            .context(UrlParseSnafu)?;

        self.client
            .get_with_token(url, None::<&()>, self.token.as_ref())
            .await
    }
}
