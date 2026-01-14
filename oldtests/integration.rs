use std::fs;

use arcgis_api_rs::{
    auth::AuthType,
    config::get_config,
    feature_layer::FeatureLayer,
    feature_layer_query::{EsriCountResponse, EsriQueryResponse, FeatureLayerQueryBuilder},
    group::create::CreateGroupQuery,
    oauth::{get_token, TokenStore},
    parser::parse_response,
};
use reqwest::Client;
use secrecy::ExposeSecret;

#[tokio::test]
async fn generate_token() {
    // configure
    let config = get_config().expect("Failed to create create config");
    let client = Client::new();

    // test
    let token = config
        .generate_access_token(&client)
        .await
        .expect("Failed to generate access token");

    assert!(!token.expose_secret().is_empty());
}

// User/Group management requires a non-app-auth token
#[tokio::test]
async fn create_group() {
    // configure
    dotenv::dotenv().ok();
    let config = get_config().expect("Failed to create create config");
    let client = config
        .build_authorized_request_client(AuthType::TestToken)
        .await
        .expect("Failed to create authorized request client");

    // test
    let uuid = uuid::Uuid::new_v4().to_string();
    let title = format!("test-{}", uuid);
    let query = CreateGroupQuery::builder(&config.portal_root, &title)
        .tags(vec!["test".to_string(), "dev".to_string()])
        // .access(arcgis_rest_api::create_group::AccessLevel::Private)
        // .privacy(arcgis_rest_api::create_group::AccessLevel::Private)
        // .contribute(arcgis_rest_api::create_group::Contributors::Owners)
        // .description("test group automation")
        // .sort_field(arcgis_rest_api::create_group::SortField::Avgrating)
        // .sort_order(arcgis_rest_api::create_group::SortOrder::Desc)
        // .is_invitation_only(true)
        // .is_view_only(true)
        // .hidden_members(true)
        // .membership_access(arcgis_rest_api::create_group::MembershipAccess::Org)
        .build();

    let create_result = query
        .send(&client)
        .await
        .expect("Failed to send create group query");

    let group = create_result.group;

    assert!(&group.title == &title);

    let delete_result = group
        .delete(&config.portal_root, &client, &group.id)
        .await
        .expect("Failed to delete group");

    assert!(delete_result.success);
    assert!(delete_result.group_id == group.id);
}

#[tokio::test]
async fn oauth() {
    let token = get_token().await.unwrap();
    // TODO: assert user profile with token

    let content = fs::read_to_string(".tokens").unwrap();
    let store: TokenStore = serde_json::from_str(&content).unwrap();

    // let entry = keyring::Entry::new("arcgis-rs", "tokens").unwrap();
    // let json = entry.get_password().unwrap();
    // let store: TokenStore = serde_json::from_str(&json).unwrap();
    assert!(!token.is_empty());
    assert!(!store.access_token.is_empty());
    assert!(!store.refresh_token.is_empty());
}

// TODO: this does not test feature updating
// #[tokio::test]
// async fn update_features() {
//     // configure
//     dotenv::dotenv().ok();
//     let fs_url = std::env::var("TEST_FEATURE_SERVICE")
//         .expect("Failed to find env variable 'TEST_FEATURE_SERVICE'");
//     let config = get_config().expect("Failed to create create config");
//     let client = config
//         .build_authorized_request_client(AuthType::None)
//         .await
//         .expect("Failed to create authorized request client");
//
//     // test
//     // let fs = FeatureLayer::new(&client, &fs_url)
//     //     .await
//     //     .expect("Failed to create feature service");
//
//     let query = FeatureLayerQueryBuilder::new()
//         .set_out_fields("*")
//         .set_return_geometry(true)
//         .build();
//     let response = query
//         .send(&client, &fs_url)
//         .await
//         .expect("Feature service query failed");
//
//     let json = parse_response::<EsriQueryResponse>(response)
//         .await
//         .expect("Failed to parse response");
//
//     println!("{:?}", json.features[0]);
//     assert!(json.features.len() > 0);
//     assert!(json.features[0].geometry.is_some());
//     assert!(json.features[0].attributes.is_object());
// }
