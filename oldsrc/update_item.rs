use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::parser::parse_response;

pub struct UpdateItemQuery {
    url: String,
    params: HashMap<String, String>,
}

#[derive(Default)]
pub struct UpdateItemQueryBuilder {
    url: String,

    title: Option<String>,
    thumbnail: Option<String>,
    thumbnail_url: Option<String>,
    metadata: Option<String>,
    type_keywords: Option<String>,
    description: Option<String>,
    tags: Option<Vec<String>>,
    snippet: Option<String>,
    extent: Option<String>,
    spatial_reference: Option<String>,
    access_information: Option<String>,
    license_info: Option<String>,
    culture: Option<String>,
    properties: Option<String>,
    item_url: Option<String>,
    service_username: Option<String>,
    service_password: Option<String>,
    service_credentials_type: Option<String>,
    service_proxy_params: Option<String>,
    service_proxy_filter: Option<String>,
    app_categories: Option<String>,
    categories: Option<String>,
    industries: Option<String>,
    languages: Option<String>,
    large_thumbnail: Option<String>,
    banner: Option<String>,
    screenshot: Option<String>,
    listing_properties: Option<String>,
    file: Option<String>,
    text: Option<String>,
    data_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateItemResponse {
    pub success: bool,
    pub id: String,
}

impl UpdateItemQuery {
    pub fn builder(
        root: impl Into<String>,
        user_name: impl Into<String>,
        id: impl Into<String>,
    ) -> UpdateItemQueryBuilder {
        UpdateItemQueryBuilder::new(root, user_name, id)
    }

    pub async fn send(&self, client: &Client) -> anyhow::Result<UpdateItemResponse> {
        let response = client.post(&self.url).form(&self.params).send().await?;
        let body = parse_response::<UpdateItemResponse>(response).await?;
        Ok(body)
    }
}

impl UpdateItemQueryBuilder {
    pub fn new(
        root: impl Into<String>,
        user_name: impl Into<String>,
        id: impl Into<String>,
    ) -> Self {
        // https://[root]/content/users/[userName]/items/[itemID]/update

        let url = format!(
            "{}/content/users/{}/items/{}/update",
            root.into(),
            user_name.into(),
            id.into()
        );
        // TODO: validtate url
        Self {
            url,
            ..Default::default()
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn thumbnail(mut self, thumbnail: impl Into<String>) -> Self {
        self.thumbnail = Some(thumbnail.into());
        self
    }

    pub fn thumbnail_url(mut self, thumbnail_url: impl Into<String>) -> Self {
        self.thumbnail_url = Some(thumbnail_url.into());
        self
    }

    pub fn metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    pub fn type_keywords(mut self, type_keywords: impl Into<String>) -> Self {
        self.type_keywords = Some(type_keywords.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }

    pub fn extent(mut self, extent: impl Into<String>) -> Self {
        self.extent = Some(extent.into());
        self
    }

    pub fn spatial_reference(mut self, spatial_reference: impl Into<String>) -> Self {
        self.spatial_reference = Some(spatial_reference.into());
        self
    }

    pub fn access_information(mut self, access_information: impl Into<String>) -> Self {
        self.access_information = Some(access_information.into());
        self
    }

    pub fn license_info(mut self, license_info: impl Into<String>) -> Self {
        self.license_info = Some(license_info.into());
        self
    }

    pub fn culture(mut self, culture: impl Into<String>) -> Self {
        self.culture = Some(culture.into());
        self
    }

    pub fn properties(mut self, properties: impl Into<String>) -> Self {
        self.properties = Some(properties.into());
        self
    }

    pub fn item_url(mut self, item_url: impl Into<String>) -> Self {
        self.item_url = Some(item_url.into());
        self
    }

    pub fn service_username(mut self, service_username: impl Into<String>) -> Self {
        self.service_username = Some(service_username.into());
        self
    }

    pub fn service_password(mut self, service_password: impl Into<String>) -> Self {
        self.service_password = Some(service_password.into());
        self
    }

    pub fn service_credentials_type(mut self, service_credentials_type: impl Into<String>) -> Self {
        self.service_credentials_type = Some(service_credentials_type.into());
        self
    }

    pub fn service_proxy_params(mut self, service_proxy_params: impl Into<String>) -> Self {
        self.service_proxy_params = Some(service_proxy_params.into());
        self
    }

    pub fn service_proxy_filter(mut self, service_proxy_filter: impl Into<String>) -> Self {
        self.service_proxy_filter = Some(service_proxy_filter.into());
        self
    }

    pub fn app_categories(mut self, app_categories: impl Into<String>) -> Self {
        self.app_categories = Some(app_categories.into());
        self
    }

    pub fn categories(mut self, categories: impl Into<String>) -> Self {
        self.categories = Some(categories.into());
        self
    }

    pub fn industries(mut self, industries: impl Into<String>) -> Self {
        self.industries = Some(industries.into());
        self
    }

    pub fn languages(mut self, languages: impl Into<String>) -> Self {
        self.languages = Some(languages.into());
        self
    }

    pub fn large_thumbnail(mut self, large_thumbnail: impl Into<String>) -> Self {
        self.large_thumbnail = Some(large_thumbnail.into());
        self
    }

    pub fn banner(mut self, banner: impl Into<String>) -> Self {
        self.banner = Some(banner.into());
        self
    }

    pub fn screenshot(mut self, screenshot: impl Into<String>) -> Self {
        self.screenshot = Some(screenshot.into());
        self
    }

    pub fn listing_properties(mut self, listing_properties: impl Into<String>) -> Self {
        self.listing_properties = Some(listing_properties.into());
        self
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn data_url(mut self, data_url: impl Into<String>) -> Self {
        self.data_url = Some(data_url.into());
        self
    }

    pub fn build(self) -> UpdateItemQuery {
        let mut params = HashMap::new();

        if let Some(desc) = self.description {
            params.insert("description".into(), desc);
        }

        if let Some(tags) = self.tags {
            params.insert("tags".into(), tags.join(","));
        }

        if let Some(snippet) = self.snippet {
            params.insert("snippet".into(), snippet);
        }

        if let Some(extent) = self.extent {
            params.insert("extent".into(), extent);
        }

        if let Some(spatial_reference) = self.spatial_reference {
            params.insert("spatialReference".into(), spatial_reference);
        }

        if let Some(access_information) = self.access_information {
            params.insert("accessInformation".into(), access_information);
        }

        if let Some(license_info) = self.license_info {
            params.insert("licenseInfo".into(), license_info);
        }

        if let Some(culture) = self.culture {
            params.insert("culture".into(), culture);
        }

        if let Some(properties) = self.properties {
            params.insert("properties".into(), properties);
        }

        if let Some(item_url) = self.item_url {
            params.insert("itemUrl".into(), item_url);
        }

        if let Some(service_username) = self.service_username {
            params.insert("serviceUsername".into(), service_username);
        }

        if let Some(service_password) = self.service_password {
            params.insert("servicePassword".into(), service_password);
        }

        if let Some(service_credentials_type) = self.service_credentials_type {
            params.insert("serviceCredentialsType".into(), service_credentials_type);
        }

        if let Some(service_proxy_params) = self.service_proxy_params {
            params.insert("serviceProxyParams".into(), service_proxy_params);
        }

        if let Some(service_proxy_filter) = self.service_proxy_filter {
            params.insert("serviceProxyFilter".into(), service_proxy_filter);
        }

        if let Some(app_categories) = self.app_categories {
            params.insert("appCategories".into(), app_categories);
        }

        if let Some(categories) = self.categories {
            params.insert("categories".into(), categories);
        }

        if let Some(industries) = self.industries {
            params.insert("industries".into(), industries);
        }

        if let Some(languages) = self.languages {
            params.insert("languages".into(), languages);
        }

        if let Some(large_thumbnail) = self.large_thumbnail {
            params.insert("largeThumbnail".into(), large_thumbnail);
        }

        if let Some(banner) = self.banner {
            params.insert("banner".into(), banner);
        }

        if let Some(screenshot) = self.screenshot {
            params.insert("screenshot".into(), screenshot);
        }

        if let Some(listing_properties) = self.listing_properties {
            params.insert("listingProperties".into(), listing_properties);
        }

        if let Some(file) = self.file {
            params.insert("file".into(), file);
        }

        if let Some(text) = self.text {
            params.insert("text".into(), text);
        }

        if let Some(data_url) = self.data_url {
            params.insert("dataUrl".into(), data_url);
        }

        params.insert("f".into(), "json".into());
        UpdateItemQuery {
            url: self.url,
            params,
        }
    }
}
