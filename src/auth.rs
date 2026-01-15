use crate::{
    error::{ReqwestSnafu, Result},
    models::TokenResponse,
};
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, RwLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use secrecy::{ExposeSecret, SecretString};
use snafu::ResultExt;

/// State used to authenticate to ArcGIS
#[derive(Clone)]
#[allow(dead_code)]
pub enum AuthState {
    /// No state
    None,

    /// This is Esri’s proprietary method that predates OAuth 2.0. It involves sending a username and password directly to the /sharing/rest/generateToken endpoint.
    /// Permissions: Inherits all privileges of the user credentials provided.
    /// Use Case: Legacy scripts or ArcGIS Enterprise environments where OAuth is not yet configured. It is generally discouraged for new development due to the security risk of handling raw passwords.
    /// Duration: Variable, typically 60 minutes.
    LegacyToken {
        auth: LegacyToken,
        token: CachedToken,
        refresh_mutex: Arc<tokio::sync::Mutex<()>>,
    },
    /// OAuth 2.0 - User Authentication (Authorization Code Flow)
    /// This method prompts a user to sign in with their ArcGIS credentials. Your app receives an access token once the user grants permission.
    /// Permissions: The token inherits all privileges of the signed-in user. If the user is an Administrator, the token has administrative rights.
    /// Use Case: Apps where users need to access their own private maps, edit data, or perform analysis using their own credits.
    /// Duration: Short-lived (usually 30 minutes to 2 hours), but can be refreshed using a refresh_token.
    OAuthUser,

    /// OAuth 2.0 - App Authentication (Client Credentials Flow)
    /// This uses a client_id and client_secret registered in your ArcGIS portal to generate a token without user interaction.
    /// Permissions: Permissions are limited to the privileges assigned to the Application Item in the portal. It cannot "impersonate" a user's private content unless that content is shared with the application.
    /// Use Case: Server-to-server communication, scheduled scripts, or public apps that need to access location services (like routing) billed to the developer's account.
    OAuthApp,

    /// API Keys are long-lived access tokens created and managed through the developer dashboard or portal content.
    /// Permissions: Scoped/Granular. You explicitly define what the key can do (e.g., "Allow Basemaps," "Allow Routing," or "Access Item X"). It does not have broad "user" permissions.
    /// Use Case: Static apps, IoT devices, or simple web maps where you don't want to implement a full OAuth login flow.
    /// Duration: Up to 1 year.
    /// Legacy Note: If you are using "Legacy API Keys" (created before June 2024), these are set to expire by May 2026. You should migrate to the new "API Key Credentials" items.
    APIKey,
}

#[derive(Debug, Clone)]
struct CachedTokenInner {
    expiration: Option<DateTime<Utc>>,
    secret: SecretString,
}

impl CachedTokenInner {
    fn new(secret: SecretString, expiration: Option<DateTime<Utc>>) -> Self {
        Self { secret, expiration }
    }

    fn expose_secret(&self) -> &str {
        self.secret.expose_secret()
    }
}

/// A cached API access token (which may be None)
pub struct CachedToken(RwLock<Option<CachedTokenInner>>);

impl CachedToken {
    pub fn clear(&self) {
        *self.0.write().unwrap() = None;
    }

    /// Returns a valid token if it exists and is not expired or if there is no expiration date.
    pub fn valid_token_with_buffer(&self, buffer: chrono::Duration) -> Option<SecretString> {
        let inner = self.0.read().unwrap();

        if let Some(token) = inner.as_ref() {
            if let Some(exp) = token.expiration {
                if exp - Utc::now() > buffer {
                    return Some(token.secret.clone());
                }
            } else {
                return Some(token.secret.clone());
            }
        }

        None
    }

    pub fn valid_token(&self) -> Option<SecretString> {
        self.valid_token_with_buffer(chrono::Duration::seconds(30))
    }

    pub fn set<S: Into<SecretString>>(&self, token: S, expiration: Option<DateTime<Utc>>) {
        *self.0.write().unwrap() = Some(CachedTokenInner::new(token.into(), expiration));
    }
}

impl fmt::Debug for CachedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.read().unwrap().fmt(f)
    }
}

impl fmt::Display for CachedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let option = self.0.read().unwrap();
        option
            .as_ref()
            .map(|s| s.expose_secret().fmt(f))
            .unwrap_or_else(|| write!(f, "<none>"))
    }
}

impl Clone for CachedToken {
    fn clone(&self) -> CachedToken {
        CachedToken(RwLock::new(self.0.read().unwrap().clone()))
    }
}

impl Default for CachedToken {
    fn default() -> CachedToken {
        CachedToken(RwLock::new(None))
    }
}

impl fmt::Debug for AuthState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthState::None => write!(f, "None"),
            AuthState::LegacyToken { auth, token, .. } => f
                .debug_struct("LegacyToken")
                .field("auth", auth)
                .field("token", token)
                .finish(),
            _ => write!(f, "<todo>"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum Auth {
    #[default]
    None,
    LegacyToken(LegacyToken),
}

#[derive(Debug, Clone)]
pub struct LegacyToken {
    username: SecretString,
    password: SecretString,
    referer: String,
    expiration: String,
}

impl LegacyToken {
    pub fn new(
        username: impl Into<SecretString>,
        password: impl Into<SecretString>,
        referer: impl Into<String>,
        expiration: impl Into<String>,
    ) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            referer: referer.into(),
            expiration: expiration.into(),
        }
    }

    pub async fn fetch_token(
        &self,
        portal: &str,
        client: &reqwest::Client,
    ) -> Result<(SecretString, Duration)> {
        // TODO: improve memory usage
        let mut params = HashMap::new();
        params.insert("username", self.username.expose_secret().to_string());
        params.insert("password", self.password.expose_secret().to_string());
        params.insert("referer", self.referer.to_string());
        params.insert("expiration", self.expiration.to_string());
        params.insert("f", "json".to_string());

        // TODO: should this be using the ArcGIS Client???
        let response = client
            .post(format!("{}/sharing/rest/generateToken", portal))
            .form(&params)
            .send()
            .await
            .context(ReqwestSnafu)?
            .json::<TokenResponse>()
            .await
            .context(ReqwestSnafu)?; // TODO: this should be JSONSnafu

        let ttl = duration_until(response.expires).unwrap_or_else(|| Duration::from_secs(60));

        Ok((response.token, ttl))
    }
}

fn duration_until(unix_ts: i64) -> Option<Duration> {
    // TODO: should I be using chrono & utc??
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
