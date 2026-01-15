mod common;
use common::*;

use once_cell::sync::Lazy;

#[serial_test::serial]
mod content_tests {
    use super::*;

    #[tokio::test]
    async fn test_add_item() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let uuid = uuid::Uuid::new_v4().to_string();
        let title = format!("testcsv-{}", uuid);

        let test_csv = r#"id,timestamp,status,temp_c,Longitude,Latitude
1025,2024-05-20T15:36:00Z,inactive,16.7,-109.33245320824183,41.39076580522106
1026,2024-05-20T16:51:00Z,maintenance,17.4,-109.47781997822837,41.67733099029833
1027,2024-05-20T17:53:00Z,inactive,30.4,-109.83293803634402,41.66539036168230
1028,2024-05-20T18:05:00Z,maintenance,23.1,-109.15503559233532,41.44373625216593"#;

        let response = client
            .content(None::<String>)
            .add_item()
            .file(test_csv)
            .set_type("CSV")
            .title(title)
            .send()
            .await
            .unwrap();

        assert!(response.success);
    }
}
