mod common;
use common::*;

use once_cell::sync::Lazy;

#[serial_test::serial]
mod portals_tests {
    use super::*;

    #[tokio::test]
    async fn test_portals_self() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let response = client
            .portals()
            .self_info()
            .send()
            .await
            .unwrap();

        // Verify the response has an id
        assert!(!response.id.is_empty(), "Portal ID should not be empty");
        println!("Portal ID: {}", response.id);
    }
}
