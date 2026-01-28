mod common;
use common::*;

use once_cell::sync::Lazy;

#[serial_test::serial]
mod integration_tests {
    use arcgis_sharing_rs::{
        models::{AddItemResponse, PublishItemResponse},
        ArcGISSharingClient,
    };

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

    async fn add_csv_item(client: &ArcGISSharingClient, uuid: &str) -> AddItemResponse {
        let title = format!("webmap-integration-test-csv-{}", uuid);

        let test_csv = r#"id,timestamp,status,temp_c,Longitude,Latitude
1025,2024-05-20T15:36:00Z,inactive,16.7,-109.33245320824183,41.39076580522106
1026,2024-05-20T16:51:00Z,maintenance,17.4,-109.47781997822837,41.67733099029833
1027,2024-05-20T17:53:00Z,inactive,30.4,-109.83293803634402,41.66539036168230
1028,2024-05-20T18:05:00Z,maintenance,23.1,-109.15503559233532,41.44373625216593"#;

        client
            .content(None::<String>)
            .add_item()
            .file(test_csv)
            .set_type("CSV")
            .title(title)
            .send()
            .await
            .unwrap()
    }

    async fn publish_csv_item(
        client: &ArcGISSharingClient,
        uuid: &str,
        csv_id: &str,
    ) -> PublishItemResponse {
        let service_item_name = format!("webmapIntegrationTestService_{}", uuid.replace("-", "_"));
        let layer_name = format!("layer_{}", uuid.replace("-", "_"));

        let builder =
            arcgis_sharing_rs::builders::publish::PublishParametersBuilder::new(&service_item_name)
                .set_coordinate_fields("Latitude", "Longitude")
                .add_string_field("status")
                .add_double_field("temp_c")
                .add_integer_field("id")
                .add_date_field("timestamp")
                .set_layer_name(layer_name)
                .build();

        client
            .item(None::<String>, csv_id)
            .publish()
            .set_publish_parameters(builder)
            .set_file_type("CSV")
            .send()
            .await
            .unwrap()
    }

    async fn add_web_map_item(
        client: &ArcGISSharingClient,
        uuid: &str,
        fs_url: &str,
    ) -> AddItemResponse {
        let web_map_name = format!("webmapIntegrationTestMap_{}", uuid.replace("-", "_"));

        let web_map = arcgis_sharing_rs::builders::webmap::WebMapBuilder::new()
            .set_extent(-109.5, 41.0, -109.0, 41.5, 4326)
            .add_feature_layer(fs_url, "my_custom_layer")
            .with_popup("Feature Information {objectid}")
            .add_popup_field("objectid", "OBJECTID", false, true)
            .add_popup_field("timestamp_", "Timestamp", false, true)
            .add_popup_field("status", "Status", false, true)
            .add_popup_field_with_format("latitude", "Latitude", true, true, 2)
            .add_popup_field_with_format("longitude", "Longitude", true, true, 2)
            .set_basemap(arcgis_sharing_rs::models::BasemapPreset::Topographic);

        client
            .content(None::<String>)
            .add_item()
            .web_map(web_map_name, web_map)
            .send()
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_full_web_map_creation_flow() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let uuid = uuid::Uuid::new_v4().to_string();
        let add_csv_response = add_csv_item(&client, &uuid).await;
        let publish_csv_response = publish_csv_item(&client, &uuid, &add_csv_response.id).await;
        let service_url = publish_csv_response.services[0].serviceurl.clone();
        let web_map_response = add_web_map_item(&client, &uuid, &service_url).await;
        assert!(web_map_response.success)
    }
}
