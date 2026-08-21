use serde::Serialize;

use crate::api::AdminFeatureServiceHandler;
use crate::{error::Result, models::AddToDefinitionResponse};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToDefinitionBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r AdminFeatureServiceHandler<'a>,

    add_to_definition: String,
}

impl<'a, 'r> AddToDefinitionBuilder<'a, 'r> {
    pub fn new(handler: &'r AdminFeatureServiceHandler<'a>, definition: serde_json::Value) -> Self {
        Self {
            handler,
            add_to_definition: serde_json::to_string(&definition).unwrap(),
        }
    }

    pub fn set_definition(mut self, definition: serde_json::Value) -> Self {
        self.add_to_definition = serde_json::to_string(&definition).unwrap();
        self
    }

    pub async fn send(&self) -> Result<AddToDefinitionResponse> {
        let url = format!("{}/addToDefinition", self.handler.url);
        self.handler.client.post(url, Some(self), None).await
    }
}
