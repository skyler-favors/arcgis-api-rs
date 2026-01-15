use arcgis_sharing_rs::ArcGISSharingClient;
use once_cell::sync::Lazy;

fn init() {
    dotenvy::dotenv().ok();

    let username = std::env::var("APP_ARCGIS_USERNAME")
        .expect("No username found; Missing env var: APP_ARCGIS_USERNAME");
    let password = std::env::var("APP_ARCGIS_PASSWORD")
        .expect("No password found; Missing env var: APP_ARCGIS_PASSWORD");
    let portal = std::env::var("APP_ARCGIS_PORTAL")
        .expect("No portal found; Missing env var: APP_ARCGIS_PORTAL");
    let referer = "127.0.0.1".to_string();
    let expiration = "1";

    let client = arcgis_sharing_rs::ArcGISSharingClient::builder()
        .portal(portal)
        .legacy_auth(username, password, referer, expiration)
        .build();
    arcgis_sharing_rs::initialise(client);
}

#[allow(dead_code)]
static SETUP: Lazy<()> = Lazy::new(|| {
    init();
});

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct TestResponse {
    //{"currentVersion":"2024.1","enterpriseVersion":"11.3.0","enterpriseBuild":"0"}
    // current_version: String,
    // enterprise_version: String,
    // enterprise_build: String,
    #[serde(flatten)]
    extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[serial_test::serial]
mod tests {
    use arcgis_sharing_rs::models::{GeometryType, SpatialRelationship};

    use super::*;

    #[tokio::test]
    async fn test_token() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let _response: TestResponse = client
            .get(
                format!("{}/sharing/rest/community/self", &client.portal),
                None::<&()>,
            )
            .await
            .unwrap();
    }

    async fn create_group(client: &ArcGISSharingClient) -> String {
        // create test group name
        let uuid = uuid::Uuid::new_v4().to_string();
        let title = format!("test-{}", uuid);

        let create_result = client
            .create_group()
            .create(&title)
            .tags(vec!["test", "dev"])
            .send()
            .await
            .expect("Failed to send create group query");

        let group = create_result.group;
        assert!(&group.title == &title);
        return group.id;
    }

    async fn delete_group(client: &ArcGISSharingClient, group_id: &str) {
        let delete_result = client.groups(group_id).delete().send().await.unwrap();
        assert!(delete_result.success);
        assert!(delete_result.group_id == group_id);
    }

    #[tokio::test]
    async fn test_group_lifecycle() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let group_id = create_group(&client).await;
        delete_group(&client, &group_id).await;
    }

    #[tokio::test]
    async fn test_private_feature_service() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let fs_url = std::env::var("TEST_PRIVATE_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PRIVATE_FEATURE_SERVICE'");
        let response = client.feature_service(fs_url).info().await.unwrap();
        assert!(response.r#type == "Feature Layer")
    }

    #[tokio::test]
    async fn test_public_feature_service() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let fs_url = std::env::var("TEST_PUBLIC_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PUBLIC_FEATURE_SERVICE'");
        let response = client.feature_service(fs_url).info().await.unwrap();
        assert!(response.r#type == "Feature Layer")
    }

    #[tokio::test]
    async fn test_feature_service_query_count_only() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let fs_url = std::env::var("TEST_PRIVATE_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PRIVATE_FEATURE_SERVICE'");
        let response = client
            .feature_service(fs_url)
            .query()
            .set_count_only(true)
            .send()
            .await
            .unwrap();
        assert!(response.count > 0)
    }

    #[tokio::test]
    async fn test_feature_service_query_point_geometry() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let fs_url = std::env::var("TEST_PUBLIC_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PUBLIC_FEATURE_SERVICE'");

        // test - create a point query with within relationship
        let geometry = r#"{"x":-119.71530713468918,"y":37.781061871461439}"#.to_string();

        let response = client
            .feature_service(fs_url)
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
    async fn test_feature_service_query_polygon_geometry() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
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
            .feature_service(fs_url)
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
    async fn test_feature_service_apply_edits_update() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let fs_url = std::env::var("TEST_PRIVATE_FEATURE_SERVICE")
            .expect("Failed to find env variable 'TEST_PRIVATE_FEATURE_SERVICE'");

        let updates = vec![
            serde_json::json!({"attributes": {"objectid": 1, "make": "Honda"}}),
            serde_json::json!({"attributes": {"objectid": 2, "make": "Honda"}}),
        ];

        let response = client
            .feature_service(fs_url)
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
}
