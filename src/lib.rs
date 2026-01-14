use crate::client::ArcGISSharingClient;
use once_cell::sync::Lazy;
use std::sync::Arc;

mod api;
mod auth;
mod client;
mod error;
mod models;

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
