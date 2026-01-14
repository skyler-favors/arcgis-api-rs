use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct EsriErrorResponse {
    pub error: EsriErrorValue,
}

#[derive(Deserialize, Debug)]
pub struct EsriErrorValue {
    pub code: i32,
    pub message: String,
    //details: Vec<String>,
}

// This is from arcgis-api-rs
pub async fn parse_response<T: DeserializeOwned>(response: Response) -> anyhow::Result<T> {
    let json = response.json::<Value>().await?;

    if let Ok(result) = serde_json::from_value::<T>(json.clone()) {
        Ok(result)
    } else if let Ok(error) = serde_json::from_value::<EsriErrorResponse>(json.clone()) {
        Err(anyhow::anyhow!("{:?}", error))
    } else {
        Err(anyhow::anyhow!("Failed to parse response: {:?}", json))
    }
}

// This lives in the pivot-rs
// pub async fn parse_response<T: DeserializeOwned>(response: Response) -> anyhow::Result<T> {
//     let json = response
//         .json::<Value>()
//         .await
//         .context("Failed to deserialize response to JSON")?;
//
//     match serde_json::from_value::<T>(json.clone()) {
//         Ok(result) => Ok(result),
//         Err(e_t) => {
//             // Try to parse it as an EsriErrorResponse for better diagnostics
//             match serde_json::from_value::<EsriErrorResponse>(json.clone()) {
//                 Ok(error) => Err(anyhow::anyhow!("Service responded with an error: {:?}", error)),
//                 Err(e_e) => Err(anyhow::anyhow!(
//                     "Failed to parse JSON into expected type: {}\nAlso failed to parse as EsriErrorResponse: {}",
//                     e_t,
//                     e_e
//                 )),
//             }
//         }
//     }
// }
