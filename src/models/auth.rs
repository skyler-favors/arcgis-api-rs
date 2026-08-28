use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub token: SecretString,
    pub expires: i64,
    #[allow(dead_code)]
    ssl: bool,
}

/// Token response for an OAuth 2.0 `authorization_code` grant.
#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthAuthorizationCodeResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub refresh_token_expires_in: u64,
    pub username: Option<String>,
    pub ssl: Option<bool>,
}

/// Token response for an OAuth 2.0 `refresh_token` grant.
#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthAccessTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub username: Option<String>,
    pub ssl: Option<bool>,
}

/// Token response for an OAuth 2.0 `exchange_refresh_token` grant.
#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthRefreshTokenExchangeResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub refresh_token_expires_in: u64,
    pub username: Option<String>,
    pub ssl: Option<bool>,
}

impl fmt::Debug for OAuthAuthorizationCodeResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAuthAuthorizationCodeResponse")
            .field("access_token", &"<redacted>")
            .field("expires_in", &self.expires_in)
            .field("refresh_token", &"<redacted>")
            .field("refresh_token_expires_in", &self.refresh_token_expires_in)
            .field("username", &self.username)
            .field("ssl", &self.ssl)
            .finish()
    }
}

impl fmt::Debug for OAuthAccessTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAuthAccessTokenResponse")
            .field("access_token", &"<redacted>")
            .field("expires_in", &self.expires_in)
            .field("username", &self.username)
            .field("ssl", &self.ssl)
            .finish()
    }
}

impl fmt::Debug for OAuthRefreshTokenExchangeResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAuthRefreshTokenExchangeResponse")
            .field("access_token", &"<redacted>")
            .field("expires_in", &self.expires_in)
            .field("refresh_token", &"<redacted>")
            .field("refresh_token_expires_in", &self.refresh_token_expires_in)
            .field("username", &self.username)
            .field("ssl", &self.ssl)
            .finish()
    }
}
