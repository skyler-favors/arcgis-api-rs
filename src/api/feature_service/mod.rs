mod apply_edits;
mod query;

use snafu::ResultExt;
use url::Url;

use crate::{
    api::feature_service::{
        apply_edits::FeatureServiceApplyEditsBuilder, query::FeatureServiceQueryBuilder,
    },
    error::{Result, UrlParseSnafu},
    models::FeatureServiceInfo,
    ArcGISSharingClient,
};
use secrecy::SecretString;

pub struct FeatureServiceHandler<'a> {
    pub(crate) client: &'a ArcGISSharingClient,
    pub(crate) url: Url,
    pub(crate) token: Option<SecretString>,
}

impl<'a> FeatureServiceHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, url: impl Into<String>) -> Self {
        // I think it's fine to unwrap here
        // We want to panic early if the url is invalid
        let url = Url::parse(&url.into()).context(UrlParseSnafu).unwrap();

        Self { client, url, token: None }
    }

    pub(crate) fn new_with_token(
        client: &'a ArcGISSharingClient,
        url: impl Into<String>,
        token: SecretString,
    ) -> Self {
        let url = Url::parse(&url.into()).context(UrlParseSnafu).unwrap();
        Self { client, url, token: Some(token) }
    }

    pub async fn info(&self) -> Result<FeatureServiceInfo> {
        self.client
            .get_with_token(self.url.as_str(), None::<&()>, self.token.as_ref())
            .await
    }

    pub fn query(&self) -> FeatureServiceQueryBuilder<'_, '_> {
        FeatureServiceQueryBuilder::new(self)
    }

    pub fn apply_edits(&self) -> FeatureServiceApplyEditsBuilder<'_, '_> {
        FeatureServiceApplyEditsBuilder::new(self)
    }
}
