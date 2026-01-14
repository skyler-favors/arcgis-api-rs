use crate::error::{Error, Result};
use async_trait::async_trait;
use bytes::Bytes;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait FromResponse: Sized {
    async fn from_response(bytes: Bytes) -> Result<Self>;
}

#[async_trait]
impl<T: DeserializeOwned> FromResponse for T {
    async fn from_response(bytes: Bytes) -> Result<Self> {
        // Ensure the HTTP status is successful (like octocrab's map_github_error)
        //let response = response.error_for_status().map_err(MyError::Http)?;

        // Use serde_path_to_error for better debugging
        let de = &mut serde_json::Deserializer::from_slice(&bytes);
        serde_path_to_error::deserialize(de).map_err(|e| Error::Json {
            source: e,
            backtrace: std::backtrace::Backtrace::capture(),
        })
    }
}
