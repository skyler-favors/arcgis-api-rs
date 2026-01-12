use reqwest::{header, Client};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::{
    collections::HashMap,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::{Mutex, RwLock};

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

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub token: String,
    pub expires: i64,
    pub ssl: bool,
}

#[derive(Clone, Debug)]
pub struct ArcGISAccessToken {
    pub value: String,
    pub expires: Instant,
}

impl ArcGISAccessToken {
    fn needs_refresh(&self, skew: Duration) -> bool {
        let now = Instant::now();
        let threshold = now + skew;
        let needs_refresh = threshold >= self.expires;

        if needs_refresh {
            let remaining = self.expires.saturating_duration_since(now);
            tracing::debug!(
                remaining_seconds = remaining.as_secs(),
                skew_seconds = skew.as_secs(),
                "Token needs refresh"
            );
        }

        needs_refresh
    }

    /// Returns the remaining time until token expires
    fn time_until_expiry(&self) -> Duration {
        self.expires.saturating_duration_since(Instant::now())
    }
}

/// Calculates the duration from now until the given Unix timestamp.
/// ArcGIS returns timestamps in milliseconds since Unix epoch.
fn duration_until(unix_ts: i64) -> Option<Duration> {
    // ArcGIS returns timestamps in milliseconds
    let target = UNIX_EPOCH.checked_add(Duration::from_millis(unix_ts as u64))?;
    let duration = target.duration_since(SystemTime::now()).ok()?;

    // Validate the duration is reasonable (between 1 minute and 15 days)
    let secs = duration.as_secs();
    if secs < 60 {
        tracing::warn!(
            unix_ts = unix_ts,
            duration_secs = secs,
            "Token TTL is less than 60 seconds, which seems unreasonable"
        );
    } else if secs > 15 * 24 * 3600 {
        tracing::warn!(
            unix_ts = unix_ts,
            duration_secs = secs,
            "Token TTL is more than 15 days, which exceeds ArcGIS maximum"
        );
    }

    Some(duration)
}

/// ArcGIS Provider
pub struct ArcGISProvider {
    pub client: reqwest::Client,
    pub portal: String,
    pub username: SecretString,
    pub password: SecretString,
    /// point to this server
    pub referer: String,
    /// in minutes; i.e. "60" for 1 hour
    pub expiration: String,
}

impl ArcGISProvider {
    pub async fn fetch_token(&self) -> anyhow::Result<(String, Duration)> {
        tracing::info!(portal = %self.portal, "Fetching new ArcGIS token");

        let mut params = HashMap::new();
        params.insert("username", self.username.expose_secret().to_string());
        params.insert("password", self.password.expose_secret().to_string());
        params.insert("referer", self.referer.to_string());
        params.insert("expiration", self.expiration.to_string());
        params.insert("f", "json".to_string());

        let response = self
            .client
            .post(format!("{}/sharing/rest/generateToken", self.portal))
            .form(&params)
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        tracing::debug!(
            expires_timestamp = response.expires,
            "Received token from ArcGIS"
        );

        let ttl = duration_until(response.expires).unwrap_or_else(|| {
            tracing::error!(
                expires_timestamp = response.expires,
                "Failed to calculate token TTL, using default 60 seconds"
            );
            Duration::from_secs(60)
        });

        tracing::info!(ttl_seconds = ttl.as_secs(), "Token fetched successfully");

        Ok((response.token, ttl))
    }
}

#[derive(Default)]
struct ArcGISTokenState {
    token: Option<ArcGISAccessToken>,
}

pub struct ArcGISTokenManager {
    state: RwLock<ArcGISTokenState>,
    refresh_gate: Mutex<()>,
    refresh_skew: Duration,
    provider: ArcGISProvider,
}

impl ArcGISTokenManager {
    pub fn new(provider: ArcGISProvider) -> Self {
        Self {
            state: RwLock::new(ArcGISTokenState::default()),
            refresh_gate: Mutex::new(()),
            refresh_skew: Duration::from_secs(5),
            provider,
        }
    }

    pub fn with_skew(mut self, skew: Duration) -> Self {
        self.refresh_skew = skew;
        self
    }

    /// Handler-facing API: cheap read-mostly path, refreshes when needed.
    pub async fn get(&self) -> anyhow::Result<String> {
        // Fast path: many readers, no mutex.
        if let Some(tok) = self.state.read().await.token.as_ref() {
            if !tok.needs_refresh(self.refresh_skew) {
                let remaining = tok.time_until_expiry();
                tracing::trace!(
                    remaining_seconds = remaining.as_secs(),
                    "Returning cached token"
                );
                return Ok(tok.value.clone());
            }

            tracing::debug!("Token needs refresh, entering slow path");
        } else {
            tracing::debug!("No token cached, fetching initial token");
        }

        // Slow path: refresh (single flight).
        self.refresh_if_needed().await?;

        // Return published token.
        let guard = self.state.read().await;
        let tok = guard.token.as_ref().ok_or_else(|| {
            tracing::error!("Token missing after refresh");
            anyhow::anyhow!("token missing after refresh")
        })?;

        tracing::debug!("Returning refreshed token");
        Ok(tok.value.clone())
    }

    async fn refresh_if_needed(&self) -> anyhow::Result<()> {
        tracing::debug!("Attempting to acquire refresh gate");

        let _gate = self.refresh_gate.lock().await;

        // Double-check after acquiring gate.
        if let Some(tok) = self.state.read().await.token.as_ref() {
            if !tok.needs_refresh(self.refresh_skew) {
                tracing::debug!("Another thread already refreshed the token");
                return Ok(());
            }
        }

        tracing::info!("Refreshing ArcGIS token");

        let (value, ttl) = self.provider.fetch_token().await?;
        let expires = Instant::now() + ttl;

        tracing::info!(
            ttl_seconds = ttl.as_secs(),
            "Token refresh successful, storing new token"
        );

        let mut w = self.state.write().await;
        w.token = Some(ArcGISAccessToken { value, expires });
        Ok(())
    }

    /// Optional: warm-up at startup.
    pub async fn warmup(&self) -> anyhow::Result<()> {
        tracing::info!("Warming up token manager");

        let _gate = self.refresh_gate.lock().await;
        let (value, ttl) = self.provider.fetch_token().await?;
        let expires = Instant::now() + ttl;

        tracing::info!(
            ttl_seconds = ttl.as_secs(),
            "Warmup complete, initial token cached"
        );

        let mut w = self.state.write().await;
        w.token = Some(ArcGISAccessToken { value, expires });
        Ok(())
    }
}
