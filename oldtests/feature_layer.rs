use arcgis_api_rs::{
    auth::AuthType,
    config::get_config,
    feature_layer::FeatureLayer,
    feature_layer_query::{
        EsriCountResponse, EsriQueryResponse, FeatureLayerQueryBuilder, GeometryType,
        QueryGeometry, SpatialRelationship,
    },
    feature_layer_update::{ApplyEditsQuery, ApplyEditsResponse},
    parser::parse_response,
};
use reqwest::Client;

// USA_States_Generalized_Boundaries
// TEST_FEATURE_SERVICE=https://services.arcgis.com/P3ePLMYs2RVChkJx/ArcGIS/rest/services/USA_States_Generalized_Boundaries/FeatureServer/0

#[tokio::test]
async fn query_feature_layer_record_count() {
    // configure
    dotenv::dotenv().ok();
    let fs_url = std::env::var("TEST_FEATURE_SERVICE")
        .expect("Failed to find env variable 'TEST_FEATURE_SERVICE'");
    let client = Client::new();

    // test
    let query = FeatureLayerQueryBuilder::new().set_count_only(true).build();
    let response = query
        .send(&client, &fs_url)
        .await
        .expect("Feature service query failed");

    let json = parse_response::<EsriCountResponse>(response)
        .await
        .expect("Failed to parse response");

    assert!(json.count == 51)
}

#[tokio::test]
async fn query_feature_layer_metadata() {
    // configure
    dotenv::dotenv().ok();
    let fs_url = std::env::var("TEST_FEATURE_SERVICE")
        .expect("Failed to find env variable 'TEST_FEATURE_SERVICE'");
    let client = Client::new();

    // test
    let fs = FeatureLayer::new(&client, &fs_url)
        .await
        .expect("Failed to create feature service");

    assert!(fs.metadata.r#type == "Feature Layer")
}

// TODO: disabling for now
// #[tokio::test]
// async fn query_with_envelope_geometry() {
//     // configure
//     dotenv::dotenv().ok();
//     let fs_url = std::env::var("TEST_FEATURE_SERVICE")
//         .expect("Failed to find env variable 'TEST_FEATURE_SERVICE'");
//     let client = Client::new();
//
//     // test - create an envelope query for a bounding box
//     let query = FeatureLayerQueryBuilder::new()
//         .set_envelope(-180.0, -90.0, 180.0, 90.0, Some(4326))
//         .set_return_geometry(true)
//         .set_out_fields("*")
//         .build();
//
//     let response = query
//         .send(&client, &fs_url)
//         .await
//         .expect("Feature service query failed");
//
//     let json = parse_response::<EsriQueryResponse>(response)
//         .await
//         .expect("Failed to parse response");
//
//     assert!(json.features.len() == 51);
// }

#[tokio::test]
async fn query_with_point_geometry() {
    // configure
    dotenv::dotenv().ok();
    let fs_url = std::env::var("TEST_FEATURE_SERVICE")
        .expect("Failed to find env variable 'TEST_FEATURE_SERVICE'");
    let client = Client::new();

    // test - create a point query with within relationship
    let geom = r#"{"x":-119.71530713468918,"y":37.781061871461439}"#.to_string();
    let query = FeatureLayerQueryBuilder::new()
        .set_geometry(geom)
        .set_spatial_relationship(SpatialRelationship::Within)
        .set_spatial_reference(4326)
        .set_geometry_type(GeometryType::Point)
        .set_return_geometry(false)
        .set_count_only(true)
        .build();

    let response = query
        .send(&client, &fs_url)
        .await
        .expect("Feature service query failed");

    let json = parse_response::<EsriCountResponse>(response)
        .await
        .expect("Failed to parse response");

    assert!(json.count == 1);
}

#[tokio::test]
async fn query_with_polygon_geometry() {
    // configure
    dotenv::dotenv().ok();
    let fs_url = std::env::var("TEST_FEATURE_SERVICE")
        .expect("Failed to find env variable 'TEST_FEATURE_SERVICE'");
    let client = Client::new();

    // test - create a polygon query
    let geom = r#"{"rings":[[[-109.39187790158928,41.419509792907284],[-101.55640533404183,41.339988469773225],[-101.78703063454039,31.004095664783694],[-109.35624516142607,31.036737940262469],[-109.39187790158928,41.419509792907284]]]}"#.to_string();

    // let rings: Vec<Vec<[f64; 2]>> = vec![vec![
    //     [-109.39187790158928, 41.419509792907284],
    //     [-101.55640533404183, 41.339988469773225],
    //     [-101.78703063454039, 31.004095664783694],
    //     [-109.35624516142607, 31.036737940262469],
    //     [-109.39187790158928, 41.419509792907284],
    // ]];

    let query = FeatureLayerQueryBuilder::new()
        .set_geometry(geom)
        .set_spatial_relationship(SpatialRelationship::Contains)
        .set_spatial_reference(4326)
        .set_geometry_type(GeometryType::Polygon)
        .set_return_geometry(false)
        .set_count_only(true)
        .build();

    let response = query
        .send(&client, &fs_url)
        .await
        .expect("Feature service query failed");

    let json = parse_response::<EsriCountResponse>(response)
        .await
        .expect("Failed to parse response");

    assert!(json.count == 2)
}

#[tokio::test]
async fn update_feature() {
    dotenv::dotenv().ok();
    let fs_url = std::env::var("TEST_FEATURE_SERVICE1")
        .expect("Failed to find env variable 'TEST_FEATURE_SERVICE1'");
    let config = get_config().expect("Failed to create create config");
    let client = config
        .build_authorized_request_client(AuthType::TestToken)
        .await
        .expect("Failed to create authorized request client");

    let updates = vec![
        serde_json::json!("{'attributes': {'objectid': 1,'make': 'Honda'}}"),
        serde_json::json!("{'attributes': {'objectid': 2,'make': 'Honda'}}"),
    ];
    let query = ApplyEditsQuery::builder().set_updates(updates).build();

    let response = query
        .send(&client, &fs_url)
        .await
        .expect("Apply edits query failed");

    let json = parse_response::<ApplyEditsResponse>(response)
        .await
        .expect("Failed to parse response");

    json.update_results
        .iter()
        .for_each(|edit| assert!(edit.success))
}
