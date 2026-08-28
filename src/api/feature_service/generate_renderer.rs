use serde::{Serialize, Serializer};
use snafu::ResultExt;

use crate::api::FeatureServiceHandler;
use crate::error::{Result, UrlParseSnafu};

fn serialize_json_value<S>(value: &serde_json::Value, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateRendererBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r FeatureServiceHandler<'a>,

    #[serde(serialize_with = "serialize_json_value")]
    classification_def: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    r#where: Option<String>,
}

impl<'a, 'r> GenerateRendererBuilder<'a, 'r> {
    pub fn new(
        handler: &'r FeatureServiceHandler<'a>,
        classification_def: serde_json::Value,
    ) -> Self {
        Self {
            handler,
            classification_def,
            r#where: None,
        }
    }

    pub fn set_where(mut self, where_clause: impl Into<String>) -> Self {
        self.r#where = Some(where_clause.into());
        self
    }

    pub async fn send(&self) -> Result<serde_json::Value> {
        let mut url = self.handler.url.clone();
        if url.cannot_be_a_base() {
            return Err(url::ParseError::RelativeUrlWithoutBase).context(UrlParseSnafu);
        }
        url.path_segments_mut()
            .expect("cannot-be-a-base URLs returned above")
            .pop_if_empty()
            .push("generateRenderer");
        self.handler.client.get(url.as_str(), Some(self)).await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::ArcGISSharingClient;

    #[test]
    fn serializes_classification_definition_as_json() {
        let client = ArcGISSharingClient::default();
        let handler = client.feature_service("https://example.com/FeatureServer/0");
        let builder = GenerateRendererBuilder::new(
            &handler,
            json!({
                "type": "classBreaksDef",
                "classificationField": "population",
                "breakCount": 5
            }),
        )
        .set_where("population > 0");

        let encoded = serde_urlencoded::to_string(&builder).unwrap();
        let fields: std::collections::HashMap<_, _> =
            url::form_urlencoded::parse(encoded.as_bytes())
                .into_owned()
                .collect();
        let classification: serde_json::Value =
            serde_json::from_str(&fields["classificationDef"]).unwrap();

        assert_eq!(classification["type"], "classBreaksDef");
        assert_eq!(classification["classificationField"], "population");
        assert_eq!(fields["where"], "population > 0");
    }

    #[test]
    fn renderer_url_appends_to_path_before_query() {
        let client = ArcGISSharingClient::default();
        let handler = client.feature_service("https://example.com/FeatureServer/0?custom=value");
        let mut url = handler.url.clone();
        url.path_segments_mut()
            .unwrap()
            .pop_if_empty()
            .push("generateRenderer");

        assert_eq!(url.path(), "/FeatureServer/0/generateRenderer");
        assert_eq!(url.query(), Some("custom=value"));
    }

    #[tokio::test]
    async fn non_base_url_returns_error() {
        let client = ArcGISSharingClient::default();
        let handler = client.feature_service("mailto:layer@example.com");
        let error = handler
            .generate_renderer(json!({ "type": "uniqueValueDef" }))
            .send()
            .await
            .unwrap_err();

        assert!(error.to_string().contains("relative URL without a base"));
    }
}
