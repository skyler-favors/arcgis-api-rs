use std::{env, fs::File, io::Write};

use dotenv::dotenv;
use oauth2::{
    reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tiny_http::{Header, Response};

use crate::token::SpecialClient;

const HTML: &str = r#"
            <html>
                <head><title>OAuth Complete</title></head>
                <body>
                    <h1>Authorization Successful</h1>
                    <p>You can close this window.</p>
                </body>
            </html>
        "#;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenStore {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

pub async fn get_token() -> anyhow::Result<String> {
    // TODO: add to config

    dotenv().ok();
    let portal_root = env::var("APP_PORTAL_ROOT")?;
    let client_id = env::var("OAUTH_CLIENT_ID")?;
    let client_secret = env::var("OAUTH_CLIENT_SECRET")?;
    let redirect_url = env::var("OAUTH_URL")?;

    let login_url = auth_url(&portal_root, &redirect_url, &client_id);

    open::that(login_url)?;

    let oauth_client = build_oauth_client(portal_root, client_id, client_secret, redirect_url);

    let server = tiny_http::Server::http("127.0.0.1:8000").unwrap();

    let request = match server.recv() {
        Ok(rq) => rq,
        Err(e) => {
            println!("error: {}", e);
            return Err(anyhow::anyhow!("{:?}", e));
        }
    };

    let url = &request.url().to_string();

    let response = Response::from_data(HTML)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap());

    request.respond(response)?;

    let code: String = url
        .split("?")
        .skip(1)
        .take(1)
        .collect::<String>()
        .split("=")
        .skip(1)
        .take(1)
        .collect();

    let token = oauth_client
        .exchange_code(AuthorizationCode::new(code))
        .add_extra_param("client_id", oauth_client.client_id().to_string())
        .request_async(async_http_client)
        .await?;

    let access_token = token.access_token().secret();
    let refresh_token = token.refresh_token().unwrap().secret();
    let store = TokenStore {
        access_token: access_token.to_string(),
        refresh_token: refresh_token.to_string(),
        expires_in: token.expires_in().unwrap().as_secs(),
    };

    let json = serde_json::to_string_pretty(&store)?;
    let mut file = File::create(".tokens")?;
    file.write_all(json.as_bytes())?;

    // let json = serde_json::to_string(&store)?;
    // let entry = keyring::Entry::new("arcgis-rs", "tokens")?;
    // entry.set_password(&json)?;

    // TODO: return user profile
    Ok(access_token.to_string())
}

fn build_oauth_client(
    portal: String,
    client_id: String,
    client_secret: String,
    url: String,
) -> SpecialClient {
    let redirect_url = format!("{}/api/auth/oauth_callback", url).to_string();

    let auth_url = AuthUrl::new(format!("{}/oauth2/authorize", portal))
        .expect("Invalid authorization endpoint URL");

    let token_url =
        TokenUrl::new(format!("{}/oauth2/token", portal)).expect("Invalid token endpoint URL");

    SpecialClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

fn auth_url(portal: &str, callback_url: &str, client_id: &str) -> String {
    let mut url = String::new();
    url.push_str(format!("{}/oauth2/authorize", portal).as_str());
    url.push_str("?client_id=");
    url.push_str(client_id);
    url.push_str("&redirect_uri=");
    url.push_str(&format!("{}/api/auth/oauth_callback", callback_url));
    url.push_str("&response_type=code");
    url.push_str("&expiration=20160");
    url
}
