mod common;
use common::*;

use once_cell::sync::Lazy;

#[serial_test::serial]
mod item_tests {
    use arcgis_sharing_rs::models::WebMapDataJson;

    use super::*;

    #[tokio::test]
    async fn test_get_item_info() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let item_id = std::env::var("TEST_ITEM_ID").unwrap();
        let item = client.item(None::<String>, &item_id).info().await.unwrap();
        assert_eq!(item.id, item_id);
        assert_eq!(item.title, "Cars");
    }

    #[tokio::test]
    async fn test_get_item_data() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let item_id = std::env::var("TEST_ITEM_ID2").unwrap();

        let data = client
            .item(None::<String>, &item_id)
            .data()
            .send::<WebMapDataJson>()
            .await
            .unwrap();

        println!(
            "Item data: {}",
            serde_json::to_string_pretty(&data).unwrap()
        );
    }

    #[tokio::test]
    async fn test_publish_csv_item_parameters() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let uuid = uuid::Uuid::new_v4().to_string();
        let title = format!("test-publish-csv-{}", uuid);
        let filename = format!("{}.csv", uuid);
        let service_name = format!("TestPublish_{}", uuid.replace("-", "_"));

        let test_csv = r#"id,timestamp,status,temp_c,Longitude,Latitude
    1025,2024-05-20T15:36:00Z,inactive,16.7,-109.33245320824183,41.39076580522106
    1026,2024-05-20T16:51:00Z,maintenance,17.4,-109.47781997822837,41.67733099029833
    1027,2024-05-20T17:53:00Z,inactive,30.4,-109.83293803634402,41.66539036168230
    1028,2024-05-20T18:05:00Z,maintenance,23.1,-109.15503559233532,41.44373625216593"#;

        // First add a CSV item
        let add_response = client
            .content(None::<String>)
            .add_item()
            .file(test_csv)
            .set_type("CSV")
            .title(&title)
            .filename(&filename)
            .send()
            .await
            .unwrap();

        assert!(add_response.success);
        println!("Added item with ID: {}", add_response.id);

        // Then publish it using the builder
        // TODO: there must be a way to infer the field types from the CSV
        let builder =
            arcgis_sharing_rs::builders::publish::PublishParametersBuilder::new(&service_name)
                .set_coordinate_fields("Latitude", "Longitude")
                .add_string_field("status", 256)
                .add_double_field("temp_c")
                .add_integer_field("id")
                .add_date_field("timestamp")
                .set_layer_name(&uuid);

        let publish_response = client
            .item(None::<String>, &add_response.id)
            .publish()
            .csv_with_parameters(builder)
            .send()
            .await
            .unwrap();

        assert!(!publish_response.services.is_empty());
        assert!(!publish_response.services[0].service_item_id.is_empty());
        assert!(!publish_response.services[0].job_id.is_empty());
        assert_eq!(publish_response.services[0].service_type, "Feature Service");
        println!(
            "Published service with item ID: {} and job ID: {}",
            publish_response.services[0].service_item_id, publish_response.services[0].job_id
        );
    }

    #[tokio::test]
    async fn test_update_item() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let uuid = uuid::Uuid::new_v4().to_string();
        let original_title = format!("test-update-{}", uuid);
        let updated_title = format!("test-updated-{}", uuid);
        let filename = format!("{}.csv", uuid);

        let test_csv = r#"id,name
1,Test Item 1
2,Test Item 2"#;

        // First add a CSV item
        let add_response = client
            .content(None::<String>)
            .add_item()
            .file(test_csv)
            .set_type("CSV")
            .title(&original_title)
            .description("Original description")
            .tags("test, original")
            .filename(&filename)
            .send()
            .await
            .unwrap();

        assert!(add_response.success);
        println!("Added item with ID: {}", add_response.id);

        // Now update the item with multiple fields
        let update_response = client
            .item(None::<String>, &add_response.id)
            .update()
            .title(&updated_title)
            .description("Updated description")
            .tags("test, updated, csv")
            .snippet("This is an updated item")
            .send()
            .await
            .unwrap();

        assert!(update_response.success);
        assert_eq!(update_response.id, add_response.id);
        println!("Updated item successfully with title: {}", updated_title);
    }

    #[tokio::test]
    async fn test_update_existing_item() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        // Use the existing test item
        let item_id = std::env::var("TEST_ITEM_ID").unwrap();

        // Get current item info
        let original_item = client.item(None::<String>, &item_id).info().await.unwrap();

        let original_title = original_item.title.clone();
        println!("Original item title: {}", original_title);

        // Update with a temporary title
        let temp_title = format!(
            "{} - Updated at {}",
            original_title,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
        );
        let update_response = client
            .item(None::<String>, &item_id)
            .update()
            .title(&temp_title)
            .snippet("Temporarily updated by test")
            .send()
            .await
            .unwrap();

        assert!(update_response.success);
        assert_eq!(update_response.id, item_id);
        println!("Updated existing item with temporary title");

        // Restore original title
        let restore_response = client
            .item(None::<String>, &item_id)
            .update()
            .title(&original_title)
            .snippet("Test completed")
            .send()
            .await
            .unwrap();

        assert!(restore_response.success);
        println!("Restored original title: {}", original_title);
    }
}
