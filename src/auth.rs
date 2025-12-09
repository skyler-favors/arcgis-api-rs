use reqwest::{header, Client};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::collections::HashMap;

use crate::{config::Settings, parser::parse_response};

pub enum AuthType {
    TestToken,
    AppAuth,
    None,
}

#[derive(Deserialize)]
pub struct AuthResponse {
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct EsriTokenResponse {
    pub access_token: SecretString,
    pub expires_in: u32,
}

pub async fn token_is_valid(client: &Client, portal: &str, token: &str) -> bool {
    let url = format!("{}/community/self?f=json&token={}", portal, token);

    let resp = client
        .get(url)
        .send()
        .await
        .expect("failed to send request");

    let json = resp.json::<AuthResponse>().await;

    json.is_ok()
}

impl Settings {
    pub async fn generate_access_token(&self, client: &Client) -> anyhow::Result<SecretString> {
        let token_endpoint = format!("{}/oauth2/token", self.portal_root);
        let mut form: HashMap<&str, &str> = HashMap::new();
        let _ = &form.insert("f", "json");
        let _ = &form.insert("client_id", &self.client_id);
        let _ = &form.insert("client_secret", &self.client_secret.expose_secret());
        let _ = &form.insert("grant_type", "client_credentials");
        let _ = &form.insert("expiration", &self.token_expiration);

        let response = client.post(token_endpoint).form(&form).send().await?;
        let result = parse_response::<EsriTokenResponse>(response).await?;

        //Ok(Secret::new(result.access_token))
        Ok(result.access_token)
    }

    pub async fn build_authorized_request_client(
        &self,
        app_auth: AuthType,
    ) -> anyhow::Result<Client> {
        let token = match app_auth {
            AuthType::TestToken => {
                let token = std::env::var("APP_TEST_TOKEN")
                    .expect("No test token found; Missing env var: APP_TEST_TOKEN");
                self.test_token
                    .clone()
                    .unwrap_or(SecretString::new(token.into()))
            }
            AuthType::AppAuth => self.generate_access_token(&Client::new()).await?,
            AuthType::None => return Ok(Client::new()),
        };

        let mut headers = header::HeaderMap::new();
        let mut auth_value =
            header::HeaderValue::from_str(&format!("Bearer {}", token.expose_secret()))?;
        auth_value.set_sensitive(true);
        headers.insert("X-Esri-Authorization", auth_value.clone());

        Ok(Client::builder().default_headers(headers).build()?)
    }
}
