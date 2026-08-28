mod common;
use common::*;

use std::sync::LazyLock;

#[serial_test::serial]
mod content_tests {
    use super::*;

    #[tokio::test]
    async fn test_add_csv_item() {
        LazyLock::force(&SETUP);
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
        LazyLock::force(&SETUP);
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
    async fn test_create_service() {
        LazyLock::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let service_name = format!("test_create_service_{}", uuid::Uuid::new_v4().simple());

        let response = client
            .content(None::<String>)
            .create_service(serde_json::json!({
                "name": service_name,
                "hasStaticData": false,
                "maxRecordCount": 1000,
                "capabilities": "Create,Delete,Query,Update,Editing"
            }))
            .set_description("Create service integration test")
            .set_tags(vec!["test", "create-service"])
            .set_snippet("Created by arcgis-sharing-rs tests")
            .send()
            .await
            .unwrap();

        assert!(response.success);
        assert_eq!(response.name.as_deref(), Some(service_name.as_str()));
        assert_eq!(response.service_type.as_deref(), Some("Feature Service"));
        assert_eq!(response.item_id, response.service_item_id);

        let item_id = response.item_id.expect("createService response item ID");
        let delete_response = client.item(item_id).delete().send().await.unwrap();
        assert!(delete_response.success);
    }

    #[tokio::test]
    async fn test_analyze_csv_item_id() {
        LazyLock::force(&SETUP);
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
        LazyLock::force(&SETUP);
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
        LazyLock::force(&SETUP);
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

    #[tokio::test]
    async fn test_add_resource_item() {
        LazyLock::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let uuid = uuid::Uuid::new_v4().to_string();
        let title = format!("test-resource-item-{}", uuid);

        // CSV does not support addResources (CONT_0090); Code Attachment does.
        let response = client
            .content(None::<String>)
            .add_item()
            .set_type("Code Attachment")
            .title(title)
            .send()
            .await
            .unwrap();

        assert!(response.success);

        let test_json = r#"{"label":"initial","version":1}"#;

        let response2 = client
            .item(&response.id)
            .add_resources()
            .file_name("config.json")
            .file(test_json)
            .access("private")
            .send()
            .await
            .unwrap();

        assert!(response2.success);

        let updated_json = test_json.replace("\"initial\"", "\"updated\"");

        let update_resources_response = client
            .item(&response.id)
            .update_resources()
            .file_name("config.json")
            .file(&updated_json)
            .access("private")
            .send()
            .await
            .unwrap();

        assert!(update_resources_response.success);

        let config_response: String = client
            .item(&response.id)
            .get_resource()
            .send("config.json")
            .await
            .unwrap();

        assert_eq!(config_response, updated_json);

        let response2 = client
            .item(&response.id)
            .add_resources()
            .file_name("metadata.json")
            .file(test_json)
            .access("inherit")
            .send()
            .await
            .unwrap();

        assert!(response2.success);

        let get_response: String = client
            .item(&response.id)
            .get_resource()
            .send("metadata.json")
            .await
            .unwrap();

        assert_eq!(get_response, test_json);

        let list = client.item(&response.id).resources().send().await.unwrap();

        assert_eq!(list.resources.len(), 2);
    }
}
