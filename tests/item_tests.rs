#![recursion_limit = "256"]
mod common;
use common::*;

use once_cell::sync::Lazy;

#[serial_test::serial]
mod item_tests {
    use super::*;

    //     #[tokio::test]
    //     async fn test_publish_csv_item_with_helper() {
    //         Lazy::force(&SETUP);
    //         let client = arcgis_sharing_rs::instance();
    //
    //         let uuid = uuid::Uuid::new_v4().to_string();
    //         let title = format!("test-publish-csv-{}", uuid);
    //         let service_name = format!("TestPublishService_{}", uuid.replace("-", "_"));
    //
    //         let test_csv = r#"id,timestamp,status,temp_c,Longitude,Latitude
    // 1025,2024-05-20T15:36:00Z,inactive,16.7,-109.33245320824183,41.39076580522106
    // 1026,2024-05-20T16:51:00Z,maintenance,17.4,-109.47781997822837,41.67733099029833
    // 1027,2024-05-20T17:53:00Z,inactive,30.4,-109.83293803634402,41.66539036168230
    // 1028,2024-05-20T18:05:00Z,maintenance,23.1,-109.15503559233532,41.44373625216593"#;
    //
    //         // First add a CSV item
    //         let add_response = client
    //             .content(None::<String>)
    //             .add_item()
    //             .file(test_csv)
    //             .set_type("CSV")
    //             .title(&title)
    //             .filename(format!("{}.csv", uuid))
    //             .send()
    //             .await
    //             .unwrap();
    //
    //         assert!(add_response.success);
    //         println!("Added item with ID: {}", add_response.id);
    //
    //         // Then publish it using the helper method
    //         let publish_response = client
    //             .item(None::<String>, &add_response.id)
    //             .publish()
    //             .csv_with_coordinates(&service_name, "Latitude", "Longitude")
    //             .send()
    //             .await
    //             .unwrap();
    //
    //         assert!(!publish_response.services.is_empty());
    //         assert!(!publish_response.services[0].service_item_id.is_empty());
    //         assert!(!publish_response.services[0].job_id.is_empty());
    //         println!(
    //             "Published service with item ID: {} and job ID: {}",
    //             publish_response.services[0].service_item_id, publish_response.services[0].job_id
    //         );
    //     }

    #[tokio::test]
    async fn test_publish_csv_item_with_manual_parameters() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let uuid = uuid::Uuid::new_v4().to_string();
        let title = format!("test-publish-csv-manual-{}", uuid);
        let filename = format!("{}.csv", uuid);
        let service_name = format!("TestPublishManual_{}", uuid.replace("-", "_"));

        //         let test_csv = r#"Longitude,Latitude
        // -109.33245320824183,41.39076580522106
        // -109.47781997822837,41.67733099029833"#;
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
        let builder = arcgis_sharing_rs::models::CSVPublishParameterBuilder::new(&service_name)
            .set_coordinate_fields("Latitude", "Longitude")
            .add_string_field("status")
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
}
