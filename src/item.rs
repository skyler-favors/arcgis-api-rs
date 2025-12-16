use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    add_item::{points_json_to_csv, AddItemQuery},
    parser::parse_response,
    publish_item::PublishItemQuery,
    update_item::UpdateItemQueryBuilder,
};

fn validate_points(input: &[Vec<f64>]) -> Result<(), &'static str> {
    input.iter().try_for_each(|v| {
        if v.len() == 2 {
            Ok(())
        } else {
            Err("Invalid inner vector length")
        }
    })
}

pub async fn create_web_map(
    portal_root: &str,
    portal_apps_root: &str,
    client: &Client,
    title: &str,
    user_name: &str,
    input_points: Vec<Vec<f64>>,
    token: String,
) -> anyhow::Result<String> {
    validate_points(&input_points).unwrap();
    let input_json = serde_json::json!({"points": input_points});
    let csv = points_json_to_csv(&input_json.to_string()).unwrap();

    let add_item_response = AddItemQuery::builder(portal_root, user_name)
        .file(csv)
        .set_type("CSV")
        .title(title.to_string())
        .token(token.clone())
        .build()
        .send(&client)
        .await?;

    let item_id = add_item_response.id;

    let publish_item_response = PublishItemQuery::builder(portal_root, user_name, item_id.clone())
        .name(title.to_string())
        .token(token.clone())
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
        .token(token)
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
    client: Client,
    pub id: String,
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
