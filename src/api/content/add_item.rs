use crate::{api::ContentHandler, error::Result, models::*};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddItemBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ContentHandler<'a>,
}

impl<'a, 'r> AddItemBuilder<'a, 'r> {
    pub fn new(handler: &'r ContentHandler<'a>) -> Self {
        Self { handler }
    }

    pub async fn send(&self) -> Result<()> {
        todo!()
    }
}
