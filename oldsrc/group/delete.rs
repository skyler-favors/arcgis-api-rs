use std::collections::HashMap;

use serde::Deserialize;

use crate::{group::create::Group, parser::parse_response};

#[derive(Deserialize)]
pub struct DeleteGroupResponse {
    pub success: bool,
    #[serde(rename = "groupId")]
    pub group_id: String,
}

impl Group {
    pub async fn delete(
        &self,
        portal: impl Into<String>,
        client: &reqwest::Client,
        group_id: impl Into<String>,
    ) -> anyhow::Result<DeleteGroupResponse> {
        let url = format!(
            "{}/community/groups/{}/delete",
            portal.into(),
            group_id.into()
        );

        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("f".into(), "json".into());

        let response = client.post(url).form(&params).send().await?;
        let body = parse_response::<DeleteGroupResponse>(response).await?;
        Ok(body)
    }
}
