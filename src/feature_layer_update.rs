use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyEditsResponse {
    pub add_results: Vec<ApplyEditsResponseResult>,
    pub update_results: Vec<ApplyEditsResponseResult>,
    pub delete_results: Vec<ApplyEditsResponseResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyEditsResponseResult {
    pub success: bool,
    pub global_id: Option<String>,
    pub object_id: Option<i64>,
}

pub struct ApplyEditsQuery {
    params: HashMap<String, String>,
}

impl ApplyEditsQuery {
    pub fn builder() -> ApplyEditsQueryBuilder {
        ApplyEditsQueryBuilder::new()
    }

    pub async fn send(
        &self,
        client: &Client,
        url: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/applyEdits", url);
        println!("params: {:?}", self.params);
        client.post(url).form(&self.params).send().await
    }
}

pub struct ApplyEditsQueryBuilder {
    adds: Vec<serde_json::Value>,
    updates: Vec<serde_json::Value>,
    deletes: Vec<serde_json::Value>,
    return_edit_results: bool,
}

impl ApplyEditsQueryBuilder {
    pub fn new() -> ApplyEditsQueryBuilder {
        ApplyEditsQueryBuilder {
            adds: vec![],
            updates: vec![],
            deletes: vec![],
            return_edit_results: true,
        }
    }

    pub fn set_adds(mut self, adds: Vec<serde_json::Value>) -> ApplyEditsQueryBuilder {
        self.adds = adds;
        self
    }

    pub fn set_updates(mut self, updates: Vec<serde_json::Value>) -> ApplyEditsQueryBuilder {
        self.updates = updates;
        self
    }

    pub fn set_deletes(mut self, deletes: Vec<serde_json::Value>) -> ApplyEditsQueryBuilder {
        self.deletes = deletes;
        self
    }

    pub fn set_return_edit_results(mut self, return_edit_results: bool) -> ApplyEditsQueryBuilder {
        self.return_edit_results = return_edit_results;
        self
    }

    pub fn build(self) -> ApplyEditsQuery {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(
            "adds".into(),
            serde_json::to_string(&self.adds)
                .unwrap_or("".to_string())
                .replace("\"", "")
                .replace(" ", ""),
        );
        params.insert(
            "updates".into(),
            serde_json::to_string(&self.updates)
                .unwrap_or("".to_string())
                .replace("\"", "")
                .replace(" ", ""),
        );
        params.insert(
            "deletes".into(),
            serde_json::to_string(&self.deletes)
                .unwrap_or("".to_string())
                .replace("\"", "")
                .replace(" ", ""),
        );
        params.insert(
            "returnEditResults".into(),
            self.return_edit_results.to_string(),
        );
        params.insert("gdbVersion".into(), "".into());
        params.insert("rollbackOnFailure".into(), "true".into());
        params.insert("useGlobalIds".into(), "false".into());
        params.insert("returnEditMoment".into(), "false".into());
        params.insert("trueCurveClient".into(), "true".into());
        params.insert("attachments".into(), "".into());
        params.insert("timeReferenceUnknownClient".into(), "false".into());
        params.insert("datumTransformation".into(), "".into());
        params.insert("editsUploadId".into(), "".into());
        params.insert("async".into(), "false".into());
        params.insert("f".into(), "json".into());

        ApplyEditsQuery { params }
    }
}
