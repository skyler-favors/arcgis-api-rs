use crate::{
    error::{
        ArcgisError, HttpSnafu, ReqwestSnafu, SerdeUrlEncodedSnafu, UriParseError, UriParseSnafu,
        UrlParseSnafu,
    },
    from_response::FromResponse,
};
use std::{
    backtrace::Backtrace,
    collections::HashMap,
    fmt,
    str::FromStr,
    sync::RwLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::{HeaderValue, StatusCode};
use reqwest::{RequestBuilder, Url};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::{
    error::{Error, Result},
    models::TokenResponse,
};

#[derive(Deserialize, Debug, Clone)]
pub struct ArcgisErrorBody {
    pub error: ArcgisErrorResponse,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArcgisErrorResponse {
    pub code: i32,
    pub message_code: Option<String>,
    pub message: String,
    pub details: Option<Vec<String>>,
}

/// Maps an ArcGIS error response to an Error if it is one.
pub async fn map_arcgis_error(response: reqwest::Response) -> Result<Bytes> {
    let body = response.bytes().await.context(ReqwestSnafu)?;

    match serde_json::from_slice(body.as_ref()) {
        Ok(ArcgisErrorBody { error }) => {
            return Err(Error::Arcgis {
                source: Box::new(ArcgisError {
                    code: error.code,
                    message_code: error.message_code,
                    message: error.message,
                    details: error.details,
                }),
                backtrace: Backtrace::capture(),
            })
        }
        Err(_e) => return Ok(body),
    }
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
    fn clear(&self) {
        *self.0.write().unwrap() = None;
    }

    /// Returns a valid token if it exists and is not expired or if there is no expiration date.
    fn valid_token_with_buffer(&self, buffer: chrono::Duration) -> Option<SecretString> {
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

    fn valid_token(&self) -> Option<SecretString> {
        self.valid_token_with_buffer(chrono::Duration::seconds(30))
    }

    fn set<S: Into<SecretString>>(&self, token: S, expiration: Option<DateTime<Utc>>) {
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

#[derive(Debug, Clone)]
pub enum AuthState {
    None,
    LegacyToken {
        auth: LegacyToken,
        token: CachedToken,
    },
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

        let ttl = duration_until(response.expires).unwrap_or_else(|| {
            // TODO: tracing should be a feature
            // tracing::error!(
            //     expires_timestamp = response.expires,
            //     "Failed to calculate token TTL, using default 60 seconds"
            // );
            Duration::from_secs(60)
        });

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

#[derive(Clone)]
pub struct ArcGISSharingClient {
    client: reqwest::Client,
    pub portal: Url,
    auth_state: AuthState,
}

impl fmt::Debug for ArcGISSharingClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArcGISSharingClient")
            .field("auth_state", &self.auth_state)
            .finish()
    }
}

// TODO: I feel like this should not exist
// due to it circumventing the builder pattern
// but it's needed for the default client
impl Default for ArcGISSharingClient {
    fn default() -> ArcGISSharingClient {
        ArcGISSharingClient {
            client: reqwest::Client::new(),
            portal: Url::parse("https://arcgis.com").expect("Invalid portal URL"),
            auth_state: AuthState::None,
        }
    }
}

impl ArcGISSharingClient {
    pub fn builder() -> ArcGISSharingClientBuilder {
        ArcGISSharingClientBuilder::default()
    }
}

impl ArcGISSharingClient {
    /// Send a `GET` request to `route` with optional query parameters, returning
    /// the body of the response.
    pub async fn get<R, A, P>(&self, route: A, parameters: Option<&P>) -> Result<R>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        R: FromResponse,
    {
        self.get_with_headers(route, parameters, None).await
    }

    /// Send a `GET` request with no additional post-processing.
    pub async fn _get(&self, uri: impl TryInto<Url>) -> Result<reqwest::Response> {
        self._get_with_headers(uri, None).await
    }

    /// Convenience method to accept any &str, and attempt to convert it to a Uri.
    /// the method also attempts to serialize any parameters into a query string, and append it to the uri.
    fn parameterized_uri<A, P>(&self, url: A, parameters: Option<&P>) -> Result<Url>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
    {
        let mut url = url.as_ref().to_string();
        if let Some(parameters) = parameters {
            if url.contains('?') {
                url = format!("{url}&");
            } else {
                url = format!("{url}?");
            }
            url = format!(
                "{}{}",
                url,
                serde_urlencoded::to_string(parameters)
                    .context(SerdeUrlEncodedSnafu)?
                    .as_str()
            );
        }
        Url::from_str(url.as_str()).context(UrlParseSnafu)
    }

    /// Send a `GET` request to `route` with optional query parameters and headers, returning
    /// the body of the response.
    pub async fn get_with_headers<R, A, P>(
        &self,
        route: A,
        parameters: Option<&P>,
        headers: Option<http::header::HeaderMap>,
    ) -> Result<R>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        R: FromResponse,
    {
        let response = self
            ._get_with_headers(self.parameterized_uri(route, parameters)?, headers)
            .await?;
        R::from_response(map_arcgis_error(response).await?).await
    }

    /// Send a `GET` request including option to set headers, with no additional post-processing.
    pub async fn _get_with_headers(
        &self,
        url: impl TryInto<Url>,
        headers: Option<http::header::HeaderMap>,
    ) -> Result<reqwest::Response> {
        let url = url
            .try_into()
            .map_err(|_| UriParseError {})
            .context(UriParseSnafu)?;

        let mut request = self.client.get(url);
        if let Some(headers) = headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }
        let request = self.build_request(request, None::<&()>)?;
        self.execute(request).await
    }

    pub fn build_request<B: Serialize + ?Sized>(
        &self,
        mut builder: RequestBuilder,
        body: Option<&B>,
    ) -> Result<reqwest::Request> {
        if let Some(body) = body {
            builder = builder.header(http::header::CONTENT_TYPE, "application/json");
            let request = builder.json(body).build().context(ReqwestSnafu)?;
            Ok(request)
        } else {
            Ok(builder
                .header(http::header::CONTENT_LENGTH, "0")
                .body("")
                .build()
                .context(ReqwestSnafu)?)
        }
    }

    async fn refresh_token_legacy_token(&self) -> Result<SecretString> {
        let (auth, cached_token) = if let AuthState::LegacyToken {
            ref auth,
            ref token,
        } = self.auth_state
        {
            (auth, token)
        } else {
            return Err(Error::LegacyAuth {
                backtrace: Backtrace::capture(),
            });
        };

        let (token, _ttl) = auth.fetch_token(self.portal.as_str(), &self.client).await?;

        // TODO: convert duration to chrono datatime
        cached_token.set(token.clone(), None);

        Ok(token)
    }

    pub async fn execute(&self, mut request: reqwest::Request) -> Result<reqwest::Response> {
        let auth_header: Option<HeaderValue> = match self.auth_state {
            AuthState::None => None,
            AuthState::LegacyToken { ref token, .. } => {
                let token = if let Some(token) = token.valid_token() {
                    token
                } else {
                    self.refresh_token_legacy_token().await?
                };
                let mut header =
                    HeaderValue::from_str(format!("Bearer {}", token.expose_secret()).as_str())
                        .map_err(http::Error::from) // How does this work?
                        .context(HttpSnafu)?;
                header.set_sensitive(true);
                Some(header)
            }
        };

        if let Some(mut auth_header) = auth_header {
            // Only set the auth_header if the authority (host) is to ArcGIS.
            // Otherwise, leave it off as we could have been redirected
            // away from ArcGIS (via follow_location_to_data()), and we don't
            // want to give our credentials to third-party services.

            if &request.url().authority() == &self.portal.authority() {
                auth_header.set_sensitive(true);
                request
                    .headers_mut()
                    .insert("X-Esri-Authorization", auth_header);
            }
        }

        // send request
        let response = self.client.execute(request).await.context(ReqwestSnafu)?;

        let status = response.status();
        if StatusCode::UNAUTHORIZED == status {
            if let AuthState::LegacyToken { ref token, .. } = self.auth_state {
                token.clear();
            }
        }

        Ok(response)
    }
}

/// # ArcGIS Sharing API Methods
impl ArcGISSharingClient {
    // pub fn actions(&self) -> actions::ActionsHandler<'_> {
    //     actions::ActionsHandler::new(self)
    // }
}

#[derive(Default)]
pub struct ArcGISSharingClientBuilder {
    portal: Option<String>,
    auth: Auth,
}

impl ArcGISSharingClientBuilder {
    pub fn new() -> Self {
        ArcGISSharingClientBuilder::default()
    }

    pub fn portal(mut self, portal: String) -> Self {
        self.portal = Some(portal);
        self
    }

    pub fn legacy_auth(
        mut self,
        username: impl Into<SecretString>,
        password: impl Into<SecretString>,
        referer: impl Into<String>,
        expiration: impl Into<String>,
    ) -> Self {
        self.auth = Auth::LegacyToken(LegacyToken {
            username: username.into(),
            password: password.into(),
            referer: referer.into(),
            expiration: expiration.into(),
        });
        self
    }

    pub fn build(self) -> ArcGISSharingClient {
        let auth = match self.auth {
            Auth::None => AuthState::None,
            Auth::LegacyToken(auth) => AuthState::LegacyToken {
                auth: auth.clone(),
                token: CachedToken::default(),
            },
        };

        // TODO: verify portal is valid arcgis portal url
        let portal =
            Url::parse(&self.portal.expect("No portal provided")).expect("Invalid portal URL");

        ArcGISSharingClient {
            client: reqwest::Client::new(),
            portal,
            auth_state: auth,
        }
    }
}
