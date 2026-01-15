use crate::{api::ItemHandler, error::Result, models::*};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishItemBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ItemHandler<'a>,
}

impl<'a, 'r> PublishItemBuilder<'a, 'r> {
    pub fn new(handler: &'r ItemHandler<'a>) -> Self {
        Self { handler }
    }

    pub async fn send(&self) -> Result<()> {
        todo!()
    }
}
