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

pub struct FeatureServiceHandler<'a> {
    client: &'a ArcGISSharingClient,
    url: Url,
}

impl<'a> FeatureServiceHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, url: impl Into<String>) -> Self {
        // I think it's fine to unwrap here
        // We want to panic early if the url is invalid
        let url = Url::parse(&url.into()).context(UrlParseSnafu).unwrap();

        Self { client, url }
    }

    pub async fn info(&self) -> Result<FeatureServiceInfo> {
        self.client.get(self.url.as_str(), None::<&()>).await
    }

    pub fn query(&self) -> FeatureServiceQueryBuilder<'_, '_> {
        FeatureServiceQueryBuilder::new(self)
    }

    pub fn apply_edits(&self) -> FeatureServiceApplyEditsBuilder<'_, '_> {
        FeatureServiceApplyEditsBuilder::new(self)
    }
}
