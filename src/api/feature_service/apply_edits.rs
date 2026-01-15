use serde::Serialize;
use snafu::ResultExt;

use crate::api::FeatureServiceHandler;
use crate::{
    error::{Result, UrlParseSnafu},
    models::*,
};

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureServiceApplyEditsBuilder<'a, 'r> {
    #[serde(skip)]
    handler: Option<&'r FeatureServiceHandler<'a>>,
}

impl<'a, 'r> FeatureServiceApplyEditsBuilder<'a, 'r> {
    pub fn new(handler: &'r FeatureServiceHandler<'a>) -> Self {
        Self {
            handler: Some(handler),
        }
    }

    pub async fn send(&self) -> Result<()> {
        todo!()
    }
}
