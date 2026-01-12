use std::sync::Arc;

use arcgis_api_rs::{
    add_item::{points_json_to_csv, AddItemQuery},
    auth::{ArcGISProvider, ArcGISTokenManager, AuthType},
    config::{get_config, Settings},
    item::{create_web_map, Item, PointWithData},
    publish_item::PublishItemQuery,
};
use std::collections::HashMap;

use once_cell::sync::Lazy;
use secrecy::ExposeSecret;

static TEST_CONFIG: Lazy<Settings> = Lazy::new(|| get_config().expect("Failed to create config"));

static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| reqwest::Client::new());

static ARCGIS_TOKEN_MANAGER: Lazy<Arc<ArcGISTokenManager>> = Lazy::new(|| {
    let provider = ArcGISProvider {
        client: HTTP_CLIENT.clone(),
        portal: TEST_CONFIG.arcgis_portal.clone(),
        username: TEST_CONFIG.arcgis_username.clone(),
        password: TEST_CONFIG.arcgis_password.clone(),
        referer: "127.0.0.1".to_string(),
        expiration: "5".to_string(),
    };

    Arc::new(ArcGISTokenManager::new(provider))
});

#[tokio::test]
async fn test_item() {
    dotenv::dotenv().ok();
    let config = get_config().expect("Failed to create create config");
    let client = config
        .build_authorized_request_client(AuthType::TestToken)
        .await
        .expect("Failed to create authorized request client");

    let test_item_id = "96c2149a83d84336b631efcb0deb6a45";

    let item = Item::new(&config.portal_root, &client, test_item_id)
        .await
        .expect("Failed to create item");

    assert!(item.id == test_item_id);
    assert!(item.data.id == test_item_id);
}

#[tokio::test]
async fn test_update_item() {
    dotenv::dotenv().ok();
    let config = get_config().expect("Failed to create create config");
    let client = config
        .build_authorized_request_client(AuthType::TestToken)
        .await
        .expect("Failed to create authorized request client");

    let test_item_id = "96c2149a83d84336b631efcb0deb6a45";
    let test_user_name = config
        .test_user_name
        .clone()
        .expect("Failed to get test user name");
    let test_tags: Vec<String> = vec!["dev".into(), "test".into()];

    let mut item = Item::new(&config.portal_root, &client, test_item_id)
        .await
        .expect("Failed to fetch item");

    assert!(item.data.id == test_item_id);
    assert!(item.data.owner == test_user_name);

    item.update(item.update_builder().tags(test_tags.clone()))
        .await
        .expect("Failed to update item");
    assert!(item.data.tags == test_tags);

    let test_tags2: Vec<String> =
        vec![vec!["dev2".into(), "test2".into()], item.data.tags.clone()].concat();
    item.update(item.update_builder().tags(test_tags2.clone()))
        .await
        .expect("Failed to update item");
    assert!(item.data.tags == test_tags2);
}

#[tokio::test]
async fn test_add_item() {
    let config = &*TEST_CONFIG;
    let token_manager = ARCGIS_TOKEN_MANAGER.clone();
    let token = token_manager.get().await.expect("Failed to get test token");

    let client = reqwest::Client::new();

    let test_user_name = config.arcgis_username.expose_secret().to_string();

    // test points in lat/long
    let test_json = serde_json::json!({"points": [[-109.39187790158928,41.419509792907284],[-101.55640533404183,41.339988469773225],[-101.78703063454039,31.004095664783694],[-109.35624516142607,31.036737940262469]]});

    let test_csv = points_json_to_csv(&test_json.to_string()).unwrap();

    let query = AddItemQuery::builder(&config.portal_root, &test_user_name)
        .file(test_csv)
        .token(token.clone())
        .set_type("CSV")
        .title("Test Data 786232".to_string())
        .build();

    let response = query
        .send(&client)
        .await
        .expect("Failed to send add item query");

    assert!(response.success);
}

#[tokio::test]
async fn test_publish_item() {
    let config = &*TEST_CONFIG;
    let token_manager = ARCGIS_TOKEN_MANAGER.clone();
    let token = token_manager.get().await.expect("Failed to get test token");
    let client = reqwest::Client::new();
    let test_user_name = config.arcgis_username.expose_secret().to_string();

    let test_item_id = "ab40df2ffd214398b9d799ea78f42bef";

    let query = PublishItemQuery::builder(&config.portal_root, &test_user_name, test_item_id)
        .name("Test_Data_786232".to_string())
        .token(token.clone())
        // .latitude_field_name("Latitude".to_string())
        // .longitude_field_name("Longitude".to_string())
        // .description("Test Data".to_string())
        .build();

    let response = query
        .send(&client)
        .await
        .expect("Failed to send publish item query");

    println!("{:?}", response);

    assert!(response.services.len() > 0);
}

#[tokio::test]
async fn test_create_web_map_from_feature_service() {
    let config = &*TEST_CONFIG;
    let token_manager = ARCGIS_TOKEN_MANAGER.clone();
    let token = token_manager.get().await.expect("Failed to get test token");
    let client = reqwest::Client::new();
    let test_user_name = config.arcgis_username.expose_secret().to_string();
    let title = "Test Map 786232".to_string();
    let fs_url = format!(
        "{}/Hosted/Test_Data_786232/FeatureServer/0",
        config.services_root
    );
    let item_id = "7c8e942758d9400f901c3f8f578c8e86";
    let json = serde_json::json!(
    {
      "operationalLayers": [
    {
      "id": "19aeae4f198-layer-2",
      "itemId": item_id,
      "title": title,
      "url": fs_url,
      "layerType": "ArcGISFeatureLayer",
      "layerDefinition": {
        "fieldConfigurations": []
      }
    }
        ],
      "baseMap": {
        "baseMapLayers": [
          {
            "id": "World_Hillshade_3689",
            "opacity": 1,
            "title": "World Hillshade",
            "url": "https://services.arcgisonline.com/arcgis/rest/services/Elevation/World_Hillshade/MapServer",
            "visibility": true,
            "layerType": "ArcGISTiledMapServiceLayer"
          },
          {
            "id": "VectorTile_6451",
            "opacity": 1,
            "title": "World Topographic Map",
            "visibility": true,
            "itemId": "7dc6cea0b1764a1f9af2e679f642f0f5",
            "layerType": "VectorTileLayer",
            "styleUrl": "https://cdn.arcgis.com/sharing/rest/content/items/7dc6cea0b1764a1f9af2e679f642f0f5/resources/styles/root.json"
          }
        ],
        "title": "Topographic"
      },
      "authoringApp": "ArcGISMapViewer",
      "authoringAppVersion": "2025.3",
      "initialState": {
        // "viewpoint": {
        //   "targetGeometry": {
        //     "spatialReference": {
        //       "latestWkid": 3857,
        //       "wkid": 102100
        //     },
        //     "xmin": -9699596.910808342,
        //     "ymin": 4265083.676083663,
        //     "xmax": -9614446.06129877,
        //     "ymax": 4360324.213326863
        //   }
        // }
      },
      "spatialReference": {
        "latestWkid": 3857,
        "wkid": 102100
      },
      "timeZone": "system",
      "version": "2.35"
    }
                );

    let query = AddItemQuery::builder(&config.portal_root, &test_user_name)
        .set_type("Web Map")
        .title(title)
        .text(json.to_string())
        .token(token.clone())
        .build();

    let response = query
        .send(&client)
        .await
        .expect("Failed to send add item query");

    assert!(response.success);
    let output_url = format!(
        "{}/mapviewer/index.html?webmap={}",
        config.portal_apps_root, response.id
    );
    println!("{}", output_url);
}

#[tokio::test]
async fn test_create_web_map_from_input() {
    let config = &*TEST_CONFIG;
    let token_manager = ARCGIS_TOKEN_MANAGER.clone();
    let token = token_manager.get().await.expect("Failed to get test token");
    let client = reqwest::Client::new();
    let test_user_name = config.arcgis_username.expose_secret().to_string();
    let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");
    let title = format!("Test_Map_{}", uuid);
    
    // Create points with associated data
    let input_points = vec![
        PointWithData {
            coordinates: [-109.39187790158928, 41.419509792907284],
            data: HashMap::from([
                ("Name".to_string(), "Point 1".to_string()),
                ("Description".to_string(), "Northwest corner".to_string()),
                ("Value".to_string(), "100".to_string()),
            ]),
        },
        PointWithData {
            coordinates: [-101.55640533404183, 41.339988469773225],
            data: HashMap::from([
                ("Name".to_string(), "Point 2".to_string()),
                ("Description".to_string(), "Northeast corner".to_string()),
                ("Value".to_string(), "200".to_string()),
            ]),
        },
        PointWithData {
            coordinates: [-101.78703063454039, 31.004095664783694],
            data: HashMap::from([
                ("Name".to_string(), "Point 3".to_string()),
                ("Description".to_string(), "Southeast corner".to_string()),
                ("Value".to_string(), "150".to_string()),
            ]),
        },
        PointWithData {
            coordinates: [-109.35624516142607, 31.036737940262469],
            data: HashMap::from([
                ("Name".to_string(), "Point 4".to_string()),
                ("Description".to_string(), "Southwest corner".to_string()),
                ("Value".to_string(), "175".to_string()),
            ]),
        },
    ];
    
    let map_url = create_web_map(
        &config.arcgis_api_root,
        &config.portal_apps_root,
        &client,
        &title,
        &test_user_name,
        input_points,
        token,
    )
    .await
    .unwrap();
    println!("{}", map_url);
    assert!(map_url.contains("webmap="));
}
