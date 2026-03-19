use secrecy::SecretString;
use serde::Serialize;
use snafu::ResultExt;

use crate::{error::Result, error::UrlParseSnafu, models::*, ArcGISSharingClient};

/// Handler for portal-related operations
pub struct PortalsHandler<'a> {
    pub(crate) client: &'a ArcGISSharingClient,
    pub(crate) token: Option<SecretString>,
}

impl<'a> PortalsHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient) -> Self {
        Self { client, token: None }
    }

    pub(crate) fn new_with_token(client: &'a ArcGISSharingClient, token: SecretString) -> Self {
        Self { client, token: Some(token) }
    }

    pub fn self_info(&self) -> PortalsSelfBuilder<'_, '_> {
        PortalsSelfBuilder::new(self)
    }
}

/// Builder for the portals/self endpoint
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalsSelfBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r PortalsHandler<'a>,
}

impl<'a, 'r> PortalsSelfBuilder<'a, 'r> {
    pub fn new(handler: &'r PortalsHandler<'a>) -> Self {
        Self { handler }
    }

    pub async fn send(&self) -> Result<PortalsSelfResponse> {
        let url = self
            .handler
            .client
            .portal
            .join("sharing/rest/portals/self")
            .context(UrlParseSnafu)?;

        self.handler
            .client
            .get_with_token(url, None::<&()>, self.handler.token.as_ref())
            .await
    }
}
