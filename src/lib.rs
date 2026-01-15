pub use crate::client::ArcGISSharingClient;
use once_cell::sync::Lazy;
use serde::Serializer;
use std::sync::Arc;

mod api;
mod auth;
mod client;
mod error;
mod from_response;
pub mod models;

#[cfg(feature = "default-client")]
static STATIC_INSTANCE: Lazy<arc_swap::ArcSwap<ArcGISSharingClient>> =
    Lazy::new(|| arc_swap::ArcSwap::from_pointee(ArcGISSharingClient::default()));

#[cfg(feature = "default-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "default-client")))]
pub fn initialise(client: ArcGISSharingClient) -> Arc<ArcGISSharingClient> {
    STATIC_INSTANCE.swap(Arc::from(client))
}

#[cfg(feature = "default-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "default-client")))]
pub fn instance() -> Arc<ArcGISSharingClient> {
    STATIC_INSTANCE.load().clone()
}

/// Serializes a Vec<String> or Vec<T: Display> as a single comma-separated string.
/// TODO: this should not be public
pub fn serialize_comma_separated<S, T>(vec: &[T], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: std::fmt::Display,
{
    let combined = vec
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(",");
    s.serialize_str(&combined)
}
