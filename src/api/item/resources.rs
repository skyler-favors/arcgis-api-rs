use crate::api::ItemHandler;
use crate::error::Result;
use crate::models::{ListResourcesResponse, SortOrder};
use serde::Serialize;
use snafu::ResultExt;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ItemHandler<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort_field: Option<SortField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<SortOrder>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SortField {
    Size,
    Created,
    Resource,
}

impl<'a, 'r> ResourcesBuilder<'a, 'r> {
    pub(crate) fn new(handler: &'a ItemHandler) -> Self {
        Self {
            handler,
            start: None,
            num: None,
            sort_field: None,
            sort_order: None,
        }
    }

    pub fn set_start(mut self, start: i64) -> Self {
        self.start = Some(start);
        self
    }

    pub fn set_num(mut self, num: i64) -> Self {
        self.num = Some(num);
        self
    }

    pub fn set_sort_field(mut self, value: SortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn set_sort_order(mut self, value: SortOrder) -> Self {
        self.sort_order = Some(value);
        self
    }

    pub async fn send(&self) -> Result<ListResourcesResponse> {
        // TODO: implement response stream

        let url = self
            .handler
            .client
            .portal
            .join(&format!(
                "sharing/rest/content/items/{}/resources",
                self.handler.id
            ))
            .context(crate::error::UrlParseSnafu)?;

        self.handler.client.get(url, Some(self)).await
    }
}
