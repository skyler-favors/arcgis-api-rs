use crate::api::serialize_comma_separated;
use crate::error::{Result, UrlParseSnafu};
use crate::{api::ContentHandler, models::CreateServiceResponse};
use serde::Serialize;
use snafu::ResultExt;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ContentHandler<'a>,

    create_parameters: String,
    output_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "Vec::is_empty"
    )]
    tags: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    snippet: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    is_view: Option<bool>,
}

impl<'a, 'r> CreateServiceBuilder<'a, 'r> {
    pub fn new(handler: &'r ContentHandler<'a>, create_parameters: serde_json::Value) -> Self {
        Self {
            handler,
            create_parameters: serde_json::to_string(&create_parameters).unwrap(),
            output_type: "featureService".to_string(),
            description: None,
            tags: Vec::new(),
            snippet: None,
            overwrite: None,
            is_view: None,
        }
    }

    pub fn set_create_parameters(mut self, create_parameters: serde_json::Value) -> Self {
        self.create_parameters = serde_json::to_string(&create_parameters).unwrap();
        self
    }

    pub fn set_output_type(mut self, output_type: impl Into<String>) -> Self {
        self.output_type = output_type.into();
        self
    }

    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn set_tags(mut self, tags: Vec<impl Into<String>>) -> Self {
        self.tags = tags.into_iter().map(Into::into).collect();
        self
    }

    pub fn set_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }

    pub fn set_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = Some(overwrite);
        self
    }

    pub fn set_is_view(mut self, is_view: bool) -> Self {
        self.is_view = Some(is_view);
        self
    }

    pub async fn send(&self) -> Result<CreateServiceResponse> {
        let url = self
            .handler
            .client
            .portal
            .join(&format!(
                "sharing/rest/content/users/{}/createService",
                self.handler.username
            ))
            .context(UrlParseSnafu)?;

        self.handler.client.post(url, Some(self), None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{models::CreateServiceResponse, ArcGISSharingClient};
    use serde_json::json;

    #[test]
    fn serializes_create_service_parameters() {
        let client = ArcGISSharingClient::default();
        let handler = ContentHandler::new(&client, "test-user".to_string());
        let builder = CreateServiceBuilder::new(
            &handler,
            json!({
                "name": "EmptyService",
                "maxRecordCount": 1000,
                "spatialReference": { "wkid": 4326 }
            }),
        )
        .set_description("New service")
        .set_tags(vec!["empty", "feature service"])
        .set_snippet("An empty service")
        .set_overwrite(false)
        .set_is_view(false);

        let serialized = serde_urlencoded::to_string(&builder).unwrap();
        let fields: std::collections::HashMap<_, _> =
            url::form_urlencoded::parse(serialized.as_bytes())
                .into_owned()
                .collect();

        assert_eq!(fields["outputType"], "featureService");
        assert_eq!(fields["description"], "New service");
        assert_eq!(fields["tags"], "empty,feature service");
        assert_eq!(fields["snippet"], "An empty service");
        assert_eq!(fields["overwrite"], "false");
        assert_eq!(fields["isView"], "false");

        let create_parameters: serde_json::Value =
            serde_json::from_str(&fields["createParameters"]).unwrap();
        assert_eq!(create_parameters["name"], "EmptyService");
        assert_eq!(create_parameters["maxRecordCount"], 1000);
        assert_eq!(create_parameters["spatialReference"]["wkid"], 4326);
    }

    #[test]
    fn deserializes_minimal_success_response() {
        let response: CreateServiceResponse = serde_json::from_value(json!({
            "success": true,
            "serviceItemId": "0123456789abcdef"
        }))
        .unwrap();

        assert!(response.success);
        assert!(response.item_id.is_none());
        assert!(response.encoded_service_url.is_none());
        assert_eq!(
            response.service_item_id.as_deref(),
            Some("0123456789abcdef")
        );
    }
}
