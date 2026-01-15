mod common;
use common::*;

use once_cell::sync::Lazy;

#[serial_test::serial]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_portal_url_and_token() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let _response: serde_json::Value = client
            .get(
                format!("{}/sharing/rest/community/self", &client.portal),
                None::<&()>,
            )
            .await
            .unwrap();
    }
}
