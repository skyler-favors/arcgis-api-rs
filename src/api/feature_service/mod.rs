mod add_to_definition;
mod apply_edits;
mod query;

use snafu::ResultExt;
use url::Url;

use crate::{
    api::feature_service::{
        add_to_definition::AddToDefinitionBuilder, apply_edits::FeatureServiceApplyEditsBuilder,
        query::FeatureServiceQueryBuilder,
    },
    error::{Result, UrlParseSnafu},
    models::FeatureServiceInfo,
    ArcGISSharingClient,
};

pub struct AdminHandler<'a> {
    client: &'a ArcGISSharingClient,
    url: Url,
}

impl<'a> AdminHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, url: Url) -> Self {
        Self { client, url }
    }

    pub fn feature_service(&self, name: impl AsRef<str>) -> AdminFeatureServiceHandler<'a> {
        let name = name.as_ref().trim_matches('/');
        assert!(!name.is_empty(), "No feature service name provided");

        let mut url = self.url.clone();
        let mut path = url
            .path_segments_mut()
            .expect("Admin URL cannot be used as a base URL");
        path.pop_if_empty().push("services");
        for segment in name.split('/') {
            path.push(segment);
        }
        path.push("FeatureServer");
        drop(path);

        AdminFeatureServiceHandler::new(self.client, url)
    }
}

pub struct AdminFeatureServiceHandler<'a> {
    pub(crate) client: &'a ArcGISSharingClient,
    pub(crate) url: Url,
}

impl<'a> AdminFeatureServiceHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, url: Url) -> Self {
        Self { client, url }
    }

    pub fn add_to_definition(
        &self,
        definition: serde_json::Value,
    ) -> AddToDefinitionBuilder<'_, '_> {
        AddToDefinitionBuilder::new(self, definition)
    }
}

pub struct FeatureServiceHandler<'a> {
    pub(crate) client: &'a ArcGISSharingClient,
    pub(crate) url: Url,
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
