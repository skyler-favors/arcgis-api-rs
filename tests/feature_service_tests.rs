mod common;
use common::*;

use arcgis_sharing_rs::models::{GeometryType, SpatialRelationship};
use std::sync::LazyLock;

#[serial_test::serial]
mod feature_service_tests {
    use super::*;

    #[tokio::test]
    async fn test_private_feature_layer() {
        LazyLock::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let fs_url = std::env::var("TEST_PRIVATE_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PRIVATE_FEATURE_SERVICE'");
        let response = client.feature_layer(fs_url).info().await.unwrap();
        assert!(response.r#type == "Feature Layer")
    }

    #[tokio::test]
    async fn test_public_feature_layer() {
        LazyLock::force(&SETUP);
        let portal = std::env::var("APP_ARCGIS_PORTAL").unwrap();
        let client = arcgis_sharing_rs::ArcGISSharingClient::builder()
            .portal(portal)
            .build();
        let fs_url = std::env::var("TEST_PUBLIC_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PUBLIC_FEATURE_SERVICE'");
        let response = client.feature_layer(fs_url).info().await.unwrap();
        assert!(response.r#type == "Feature Layer")
    }

    #[tokio::test]
    async fn test_feature_layer_query_count_only() {
        LazyLock::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let fs_url = std::env::var("TEST_PRIVATE_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PRIVATE_FEATURE_SERVICE'");
        let response = client
            .feature_layer(fs_url)
            .query()
            .set_count_only(true)
            .send()
            .await
            .unwrap();
        assert!(response.count > 0)
    }

    #[tokio::test]
    async fn test_feature_layer_query_point_geometry() {
        LazyLock::force(&SETUP);
        let portal = std::env::var("APP_ARCGIS_PORTAL").unwrap();
        let client = arcgis_sharing_rs::ArcGISSharingClient::builder()
            .portal(portal)
            .build();
        let fs_url = std::env::var("TEST_PUBLIC_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PUBLIC_FEATURE_SERVICE'");

        // test - create a point query with within relationship
        let geometry = r#"{"x":-119.71530713468918,"y":37.781061871461439}"#.to_string();

        let response = client
            .feature_layer(fs_url)
            .query()
            .set_geometry(geometry)
            .set_spatial_reference(4326)
            .set_spatial_relationship(SpatialRelationship::Within)
            .set_geometry_type(GeometryType::Point)
            .set_return_geometry(false)
            .set_count_only(true)
            .send()
            .await
            .unwrap();

        assert!(response.count == 1);
    }

    #[tokio::test]
    async fn test_feature_layer_query_polygon_geometry() {
        LazyLock::force(&SETUP);
        let portal = std::env::var("APP_ARCGIS_PORTAL").unwrap();
        let client = arcgis_sharing_rs::ArcGISSharingClient::builder()
            .portal(portal)
            .build();
        let fs_url = std::env::var("TEST_PUBLIC_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PUBLIC_FEATURE_SERVICE'");

        // test - create a polygon query
        let geometry = r#"{"rings":[[[-109.39187790158928,41.419509792907284],[-101.55640533404183,41.339988469773225],[-101.78703063454039,31.004095664783694],[-109.35624516142607,31.036737940262469],[-109.39187790158928,41.419509792907284]]]}"#.to_string();

        // let rings: Vec<Vec<[f64; 2]>> = vec![vec![
        //     [-109.39187790158928, 41.419509792907284],
        //     [-101.55640533404183, 41.339988469773225],
        //     [-101.78703063454039, 31.004095664783694],
        //     [-109.35624516142607, 31.036737940262469],
        //     [-109.39187790158928, 41.419509792907284],
        // ]];

        let response = client
            .feature_layer(fs_url)
            .query()
            .set_geometry(geometry)
            .set_spatial_reference(4326)
            .set_spatial_relationship(SpatialRelationship::Contains)
            .set_geometry_type(GeometryType::Polygon)
            .set_return_geometry(false)
            .set_count_only(true)
            .send()
            .await
            .unwrap();

        assert!(response.count == 2)
    }

    #[tokio::test]
    async fn test_feature_layer_apply_edits_update() {
        LazyLock::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let fs_url = std::env::var("TEST_PRIVATE_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PRIVATE_FEATURE_SERVICE'");

        let updates = vec![
            serde_json::json!({"attributes": {"objectid": 10, "make": "Honda"}}),
            serde_json::json!({"attributes": {"objectid": 11, "make": "Honda"}}),
        ];

        let response = client
            .feature_layer(fs_url)
            .apply_edits()
            .set_updates(updates)
            .send()
            .await
            .unwrap();

        response
            .update_results
            .iter()
            .for_each(|edit| assert!(edit.success))
    }

    #[tokio::test]
    async fn test_feature_layer_unique_values() {
        LazyLock::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let fs_url = std::env::var("TEST_PRIVATE_FEATURE_SERVICE2")
            .expect("Failed to find env variable 'TEST_PRIVATE_FEATURE_SERVICE'");

        let response = client
            .feature_layer(fs_url)
            .query()
            .set_where("1=1")
            .set_out_fields("industry")
            .set_return_distinct_values(true)
            .send()
            .await
            .unwrap();

        let features = response.features.unwrap();

        assert!(!features.is_empty());

        let unique_values: Vec<String> = features
            .iter()
            .map(|feature| {
                feature
                    .attributes
                    .get("industry")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string()
            })
            .collect();

        println!("Unique values: {:?}", unique_values);
    }
}
