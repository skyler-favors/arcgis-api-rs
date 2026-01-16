use crate::{api::ItemHandler, error::Result, models::*};
use reqwest::multipart::Form;
use serde::Serialize;
use snafu::ResultExt;
use url::form_urlencoded;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishItemBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ItemHandler<'a>,

    itemid: String,
    filetype: String,
    publish_parameters: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    build_initial_cache: Option<bool>,
}

impl<'a, 'r> PublishItemBuilder<'a, 'r> {
    pub fn new(handler: &'r ItemHandler<'a>) -> Self {
        Self {
            handler,
            itemid: handler.id.clone(),
            filetype: String::new(),
            publish_parameters: String::new(),
            output_type: None,
            build_initial_cache: None,
        }
    }

    /// Set the file type being published (e.g., "csv", "shapefile", "geojson")
    pub fn set_file_type(mut self, filetype: impl Into<String>) -> Self {
        self.filetype = filetype.into();
        self
    }

    /// Set the publish parameters as a JSON object
    pub fn set_publish_parameters(mut self, params: serde_json::Value) -> Self {
        self.publish_parameters = serde_json::to_string(&params).unwrap();
        self
    }

    /// Set the output service type (e.g., "Tiles", "3DTilesService")
    pub fn set_output_type(mut self, output_type: impl Into<String>) -> Self {
        self.output_type = Some(output_type.into());
        self
    }

    /// Enable building initial cache for tiled services
    pub fn set_build_initial_cache(mut self, build: bool) -> Self {
        self.build_initial_cache = Some(build);
        self
    }

    /// Helper method to publish a CSV file with coordinate fields
    ///
    /// # Arguments
    /// * `name` - Name for the published service
    /// * `lat_field` - Name of the latitude field in the CSV
    /// * `lon_field` - Name of the longitude field in the CSV
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient, item_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = client
    ///     .item(None::<String>, item_id)
    ///     .publish()
    ///     .csv_with_coordinates("MyService", "Latitude", "Longitude")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    // pub fn csv_with_coordinates(
    //     mut self,
    //     name: impl Into<String>,
    //     lat_field: impl Into<String>,
    //     lon_field: impl Into<String>,
    // ) -> Self {
    //     let lat_field = lat_field.into();
    //     let lon_field = lon_field.into();
    //     let name = name.into();
    //
    //     self.filetype = "csv".to_string();
    //     self.publish_parameters = serde_json::json!({
    //         "type": "csv",
    //         "name": name,
    //         "locationType": "coordinates",
    //         "latitudeFieldName": lat_field,
    //         "longitudeFieldName": lon_field,
    //         "coordinateFieldType": "LatitudeAndLongitude",
    //         "sourceSR": {
    //             "wkid": 4326,
    //             "latestWkid": 4326
    //         },
    //         "targetSR": {
    //             "wkid": 102100,
    //             "latestWkid": 3857
    //         }
    //     });
    //     self
    // }

    /// Helper method to publish a shapefile
    ///
    /// # Arguments
    /// * `name` - Name for the published service
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient, item_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = client
    ///     .item(None::<String>, item_id)
    ///     .publish()
    ///     .shapefile("MyShapefileService")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    // pub fn shapefile(mut self, name: impl Into<String>) -> Self {
    //     self.filetype = "shapefile".to_string();
    //     self.publish_parameters = serde_json::json!({
    //         "name": name.into(),
    //         "targetSR": {
    //             "wkid": 102100,
    //             "latestWkid": 3857
    //         }
    //     });
    //     self
    // }

    /// Helper method to publish a GeoJSON file
    ///
    /// # Arguments
    /// * `name` - Name for the published service
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient, item_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = client
    ///     .item(None::<String>, item_id)
    ///     .publish()
    ///     .geojson("MyGeoJSONService")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    // pub fn geojson(mut self, name: impl Into<String>) -> Self {
    //     self.filetype = "geojson".to_string();
    //     self.publish_parameters = serde_json::json!({
    //         "name": name.into(),
    //         "targetSR": {
    //             "wkid": 102100,
    //             "latestWkid": 3857
    //         }
    //     });
    //     self
    // }

    /// Helper method to publish a service definition file
    ///
    /// Service definition files contain all the necessary information for publishing
    /// and typically don't require additional parameters.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient, item_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = client
    ///     .item(None::<String>, item_id)
    ///     .publish()
    ///     .service_definition()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    // pub fn service_definition(mut self) -> Self {
    //     self.filetype = "serviceDefinition".to_string();
    //     self.publish_parameters = serde_json::json!({});
    //     self
    // }

    fn to_multipart(&self) -> Result<Form> {
        let mut form = Form::new();

        // Serialize all fields (except handler which has #[serde(skip)])
        // The file field will be serialized as a string, which we'll exclude below
        let serialized =
            serde_urlencoded::to_string(self).context(crate::error::SerdeUrlEncodedSnafu)?;

        // Parse back as key-value pairs and add each as text to the form
        for (key, value) in form_urlencoded::parse(serialized.as_bytes()) {
            form = form.text(key.into_owned(), value.into_owned());
        }

        Ok(form)
    }

    pub async fn send(&self) -> Result<PublishItemResponse> {
        let url = self
            .handler
            .client
            .portal
            .join(&format!(
                "sharing/rest/content/users/{}/publish",
                self.handler.username
            ))
            .context(crate::error::UrlParseSnafu)?;

        let form = self.to_multipart()?;

        // POST with params as the body (not query parameters)
        self.handler.client.post_multipart(url, form).await
    }
}
