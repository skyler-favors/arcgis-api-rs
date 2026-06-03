use secrecy::SecretString;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub token: SecretString,
    pub expires: i64,
    #[allow(dead_code)]
    ssl: bool,
}

/// OAuth 2.0 authorization-code token response from `/sharing/rest/oauth2/token`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub username: Option<String>,
}
