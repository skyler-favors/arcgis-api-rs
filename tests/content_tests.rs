mod common;
use common::*;

use once_cell::sync::Lazy;

#[serial_test::serial]
mod content_tests {
    use super::*;

    #[tokio::test]
    async fn test_add_csv_item() {
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

    #[tokio::test]
    async fn test_add_webmap_item() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let uuid = uuid::Uuid::new_v4().to_string();
        let web_map_name = format!("TestWebMap_{}", uuid.replace("-", "_"));

        let fs_url = std::env::var("TEST_PRIVATE_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PRIVATE_FEATURE_SERVICE'");

        // Build web map using the builder pattern
        let web_map = arcgis_sharing_rs::builders::WebMapBuilder::new()
            .add_feature_layer(&fs_url, "cars")
            .with_popup("Feature Information {objectid}")
            .add_popup_field("objectid", "OBJECTID", false, true)
            .add_popup_field("make", "Make", false, true)
            // .add_popup_field_with_format("latitude", "Latitude", true, true, 2)
            // .add_popup_field_with_format("longitude", "Longitude", true, true, 2)
            .set_basemap(arcgis_sharing_rs::models::BasemapPreset::Topographic);

        let response = client
            .content(None::<String>)
            .add_item()
            .web_map(web_map_name, web_map)
            .send()
            .await
            .unwrap();

        assert!(response.success);
    }

    #[tokio::test]
    async fn test_analyze_csv_item_id() {
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

        let response = client
            .content(None::<String>)
            .analyze()
            .set_item_id(response.id)
            .set_filetype("csv")
            .send()
            .await
            .unwrap();

        println!("{:?}", response);

        // Verify the response contains publish parameters
        assert!(!response.publish_parameters.is_null());
    }

    #[tokio::test]
    async fn test_analyze_csv_text() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let test_csv = r#"id,timestamp,status,temp_c,Longitude,Latitude
1025,2024-05-20T15:36:00Z,inactive,16.7,-109.33245320824183,41.39076580522106
1026,2024-05-20T16:51:00Z,maintenance,17.4,-109.47781997822837,41.67733099029833
1027,2024-05-20T17:53:00Z,inactive,30.4,-109.83293803634402,41.66539036168230
1028,2024-05-20T18:05:00Z,maintenance,23.1,-109.15503559233532,41.44373625216593"#;

        let response = client
            .content(None::<String>)
            .analyze()
            .set_text(test_csv)
            .set_filetype("csv")
            .send()
            .await
            .unwrap();

        println!("{:?}", response);

        // Verify the response contains publish parameters
        assert!(!response.publish_parameters.is_null());
    }

    #[tokio::test]
    async fn test_analyze_csv_file() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let test_csv = r#"id,timestamp,status,temp_c,Longitude,Latitude
1025,2024-05-20T15:36:00Z,inactive,16.7,-109.33245320824183,41.39076580522106
1026,2024-05-20T16:51:00Z,maintenance,17.4,-109.47781997822837,41.67733099029833
1027,2024-05-20T17:53:00Z,inactive,30.4,-109.83293803634402,41.66539036168230
1028,2024-05-20T18:05:00Z,maintenance,23.1,-109.15503559233532,41.44373625216593"#;

        let response = client
            .content(None::<String>)
            .analyze()
            .set_file_content(test_csv)
            .set_filename("test.csv")
            .send()
            .await
            .unwrap();

        // Verify the response contains publish parameters
        assert!(!response.publish_parameters.is_null());
    }
}
