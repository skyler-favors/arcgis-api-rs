use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    add_item::{points_json_to_csv, AddItemQuery},
    parser::parse_response,
    publish_item::PublishItemQuery,
    update_item::UpdateItemQueryBuilder,
};

#[tokio::test]
async fn test_item() {
    use crate::auth::AuthType;
    use crate::config::get_config;

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
    use crate::auth::AuthType;
    use crate::config::get_config;

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
    use crate::auth::AuthType;
    use crate::config::get_config;

    dotenv::dotenv().ok();
    let config = get_config().expect("Failed to create create config");
    let client = config
        .build_authorized_request_client(AuthType::TestToken)
        .await
        .expect("Failed to create authorized request client");

    let test_user_name = config
        .test_user_name
        .clone()
        .expect("Failed to get test user name");

    // test points in lat/long
    let test_json = serde_json::json!({"points": [[-109.39187790158928,41.419509792907284],[-101.55640533404183,41.339988469773225],[-101.78703063454039,31.004095664783694],[-109.35624516142607,31.036737940262469]]});

    let test_csv = points_json_to_csv(&test_json.to_string()).unwrap();

    let query = AddItemQuery::builder(&config.portal_root, &test_user_name)
        .file(test_csv)
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
    use crate::auth::AuthType;
    use crate::config::get_config;

    dotenv::dotenv().ok();
    let config = get_config().expect("Failed to create create config");
    let client = config
        .build_authorized_request_client(AuthType::TestToken)
        .await
        .expect("Failed to create authorized request client");

    let test_user_name = config
        .test_user_name
        .clone()
        .expect("Failed to get test user name");

    let test_item_id = "617487df6233468f8574d161eab8caa1";

    let query = PublishItemQuery::builder(&config.portal_root, &test_user_name, test_item_id)
        .name("Test_Data_786232".to_string())
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
    use crate::auth::AuthType;
    use crate::config::get_config;

    dotenv::dotenv().ok();
    let config = get_config().expect("Failed to create create config");
    let client = config
        .build_authorized_request_client(AuthType::TestToken)
        .await
        .expect("Failed to create authorized request client");

    let test_user_name = config
        .test_user_name
        .clone()
        .expect("Failed to get test user name");

    let title = "Test Map 786232".to_string();
    let fs_url = format!(
        "{}/Hosted/Test_Data_786232/FeatureServer/0",
        config.services_root
    );
    let item_id = "cc80db296d5d4b05ba191b136d5c6bb9";
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
    use crate::auth::AuthType;
    use crate::config::get_config;

    dotenv::dotenv().ok();
    let config = get_config().expect("Failed to create create config");
    let client = config
        .build_authorized_request_client(AuthType::TestToken)
        .await
        .expect("Failed to create authorized request client");
    let test_user_name = config
        .test_user_name
        .clone()
        .expect("Failed to get test user name");
    let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");
    let title = format!("Test_Map_{}", uuid);
    let input_points = vec![
        [-109.39187790158928, 41.419509792907284],
        [-101.55640533404183, 41.339988469773225],
        [-101.78703063454039, 31.004095664783694],
        [-109.35624516142607, 31.036737940262469],
    ];
    let map_url = create_web_map(
        &config.portal_root,
        &config.portal_apps_root,
        &client,
        &title,
        &test_user_name,
        input_points,
    )
    .await
    .unwrap();
    println!("{}", map_url);
    assert!(map_url.contains("webmap="));
}

pub async fn create_web_map(
    portal_root: &str,
    portal_apps_root: &str,
    client: &Client,
    title: &str,
    user_name: &str,
    input_points: Vec<[f64; 2]>,
) -> anyhow::Result<String> {
    let input_json = serde_json::json!({"points": input_points});
    let csv = points_json_to_csv(&input_json.to_string()).unwrap();

    let add_item_response = AddItemQuery::builder(portal_root, user_name)
        .file(csv)
        .set_type("CSV")
        .title(title.to_string())
        .build()
        .send(&client)
        .await?;

    let item_id = add_item_response.id;

    let publish_item_response = PublishItemQuery::builder(portal_root, user_name, item_id.clone())
        .name(title.to_string())
        .build()
        .send(&client)
        .await?;

    assert!(publish_item_response.services.len() == 1);
    let service = publish_item_response.services.first().unwrap();

    let fs_url = format!("{}/0", service.encoded_service_url.clone());

    let map_json = serde_json::json!(
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

    let create_web_map_response = AddItemQuery::builder(portal_root, user_name)
        .set_type("Web Map")
        .title(title)
        .text(map_json.to_string())
        .build()
        .send(&client)
        .await?;

    let map_url = format!(
        "{}/mapviewer/index.html?webmap={}",
        portal_apps_root, create_web_map_response.id
    );

    Ok(map_url)
}

// TODO: check job status
// async fn check_job_status(config: &Settings, client: &Client, job_id: &str) -> anyhow::Result<()> {
//     let mut status = String::new();
//     loop {
//         let job_status = JobStatusQuery::builder(&config.portal_root, job_id)
//             .build()
//             .send(&client)
//             .await?;
//         status = job_status.status;
//         if status == "succeeded" {
//             break;
//         }
//         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//     }
//     Ok(())
// }

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemData {
    pub id: String,
    pub owner: String,
    pub title: String,
    pub created: u64,
    pub modified: u64,
    pub type_keywords: Vec<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub is_org_item: bool,
    pub categories: Vec<String>,
    pub url: Option<String>,
    pub thumbnail: Option<String>,
}

pub struct Item {
    root: String,
    id: String,
    client: Client,
    pub data: ItemData,
}

impl Item {
    pub async fn new(
        root: impl Into<String>,
        client: &Client,
        id: impl Into<String>,
    ) -> anyhow::Result<Self> {
        // Fetch item

        let root = root.into();
        let id = id.into();
        let item_data = Self::fetch_item_data(&root, &id, client).await?;

        Ok(Self {
            root,
            id,
            client: client.clone(),
            data: item_data,
        })
    }

    async fn fetch_item_data(root: &str, id: &str, client: &Client) -> anyhow::Result<ItemData> {
        let url = format!("{}/content/items/{}?f=json", &root, &id);
        let response = client.get(url).send().await?;
        let item_data = parse_response::<ItemData>(response).await?;
        Ok(item_data)
    }

    pub fn update_builder(&self) -> UpdateItemQueryBuilder {
        UpdateItemQueryBuilder::new(&self.root, &self.data.owner, &self.id)
    }

    pub async fn update(&mut self, builder: UpdateItemQueryBuilder) -> anyhow::Result<()> {
        let response = builder
            .build()
            .send(&self.client)
            .await
            .expect("Failed to send update item query");

        if !response.success {
            return Err(anyhow::anyhow!("Failed to update item"));
        }

        // update self
        let item_data = Self::fetch_item_data(&self.root, &self.id, &self.client).await?;
        self.data = item_data;

        Ok(())
    }
}
