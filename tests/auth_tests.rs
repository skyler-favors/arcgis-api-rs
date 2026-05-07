mod common;
use common::*;

use once_cell::sync::Lazy;

#[serial_test::serial]
mod feature_service_tests {
    use super::*;

    #[tokio::test]
    async fn test_private_feature_service() {
        Lazy::force(&SETUP);

        let portal = std::env::var("APP_ARCGIS_PORTAL")
            .expect("Failed to find env variable 'APP_ARCGIS_PORTAL'");

        let client = arcgis_sharing_rs::ArcGISSharingClient::builder()
            .portal(portal)
            .build();

        let fs_url = std::env::var("TEST_PRIVATE_FEATURE_SERVICE2")
            .expect("Failed to find env variable 'TEST_PRIVATE_FEATURE_SERVICE2'");

        let token = std::env::var("TEST_TOKEN").expect("Failed to find env variable 'TEST_TOKEN'");

        let response = client
            .with_token(token)
            .feature_service(fs_url)
            .info()
            .await
            .unwrap();

        assert!(response.r#type == "Feature Layer")
    }
}
