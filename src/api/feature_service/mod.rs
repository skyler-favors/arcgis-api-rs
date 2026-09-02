mod add_to_definition;
mod apply_edits;
mod generate_renderer;
mod query;

use snafu::ResultExt;
use url::Url;

use crate::{
    api::feature_service::{
        add_to_definition::AddToDefinitionBuilder, apply_edits::FeatureLayerApplyEditsBuilder,
        generate_renderer::GenerateRendererBuilder, query::FeatureLayerQueryBuilder,
    },
    error::{Result, UrlParseSnafu},
    models::{FeatureLayer, FeatureLayerInfo, FeatureServiceInfo},
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

    pub async fn layers(&self) -> Result<Vec<FeatureLayer>> {
        Ok(self.info().await?.layers)
    }

    pub fn layer(&self, id: u32) -> FeatureLayerHandler<'a> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .expect("Feature service URL cannot be used as a base URL")
            .pop_if_empty()
            .push(&id.to_string());
        FeatureLayerHandler::from_url(self.client, url)
    }
}

pub struct FeatureLayerHandler<'a> {
    pub(crate) client: &'a ArcGISSharingClient,
    pub(crate) url: Url,
}

impl<'a> FeatureLayerHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, url: impl Into<String>) -> Self {
        let url = Url::parse(&url.into()).context(UrlParseSnafu).unwrap();
        Self::from_url(client, url)
    }

    fn from_url(client: &'a ArcGISSharingClient, url: Url) -> Self {
        Self { client, url }
    }

    pub async fn info(&self) -> Result<FeatureLayerInfo> {
        self.client.get(self.url.as_str(), None::<&()>).await
    }

    pub fn query(&self) -> FeatureLayerQueryBuilder<'_, '_> {
        FeatureLayerQueryBuilder::new(self)
    }

    pub fn generate_renderer(
        &self,
        classification_def: serde_json::Value,
    ) -> GenerateRendererBuilder<'_, '_> {
        GenerateRendererBuilder::new(self, classification_def)
    }

    pub fn apply_edits(&self) -> FeatureLayerApplyEditsBuilder<'_, '_> {
        FeatureLayerApplyEditsBuilder::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_layer_appends_id_to_service_url() {
        let client = ArcGISSharingClient::default();
        let service = client.feature_service("https://example.com/FeatureServer?custom=value");

        let layer = service.layer(3);

        assert_eq!(layer.url.path(), "/FeatureServer/3");
        assert_eq!(layer.url.query(), Some("custom=value"));
    }

    #[test]
    fn service_info_describes_discoverable_layers_and_tables() {
        let info: FeatureServiceInfo = serde_json::from_value(serde_json::json!({
            "layers": [{
                "id": 0,
                "name": "Parcels",
                "parentLayerId": -1,
                "geometryType": "esriGeometryPolygon"
            }],
            "tables": [{ "id": 1, "name": "Owners" }]
        }))
        .unwrap();

        assert_eq!(info.layers[0].id, 0);
        assert_eq!(info.layers[0].name, "Parcels");
        assert_eq!(info.tables[0].id, 1);
    }
}
