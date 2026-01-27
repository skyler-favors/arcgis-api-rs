use crate::{
    api::*,
    auth::{Auth, AuthState, CachedToken, LegacyToken},
    error::{
        ArcgisError, Error, HttpSnafu, ReqwestSnafu, Result, SerdeUrlEncodedSnafu, UriParseError,
        UriParseSnafu, UrlParseSnafu,
    },
    from_response::FromResponse,
};

use bytes::Bytes;
use http::{HeaderValue, StatusCode};
use once_cell::sync::Lazy;
use reqwest::RequestBuilder;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::{backtrace::Backtrace, fmt, str::FromStr, sync::Arc};
use url::Url;

mod api;
mod auth;
pub mod builders;
mod error;
mod from_response;
pub mod models;

#[cfg(feature = "default-client")]
static STATIC_INSTANCE: Lazy<arc_swap::ArcSwap<ArcGISSharingClient>> =
    Lazy::new(|| arc_swap::ArcSwap::from_pointee(ArcGISSharingClient::default()));

#[cfg(feature = "default-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "default-client")))]
pub fn initialise(client: ArcGISSharingClient) -> Arc<ArcGISSharingClient> {
    STATIC_INSTANCE.swap(Arc::from(client))
}

#[cfg(feature = "default-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "default-client")))]
pub fn instance() -> Arc<ArcGISSharingClient> {
    STATIC_INSTANCE.load().clone()
}

#[derive(Deserialize, Debug, Clone)]
struct ArcgisErrorBody {
    error: ArcgisErrorResponse,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ArcgisErrorResponse {
    code: i32,
    message_code: Option<String>,
    message: String,
    details: Option<Vec<String>>,
}

/// Maps an ArcGIS error response to an Error if it is one.
async fn map_arcgis_error(response: reqwest::Response) -> Result<Bytes> {
    let body = response.bytes().await.context(ReqwestSnafu)?;

    match serde_json::from_slice(body.as_ref()) {
        Ok(ArcgisErrorBody { error }) => Err(Error::Arcgis {
            source: Box::new(ArcgisError {
                code: error.code,
                message_code: error.message_code,
                message: error.message,
                details: error.details,
            }),
            backtrace: Backtrace::capture(),
        }),
        Err(_e) => Ok(body),
    }
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
            portal: Url::parse("https://arcgis.com/").expect("Invalid portal URL"),
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
    /// Send a `POST` request to `route` with an optional body, returning the body
    /// of the response.
    pub async fn post<P: Serialize + ?Sized, R: FromResponse>(
        &self,
        route: impl AsRef<str>,
        parameters: Option<&P>,
        body: Option<&P>,
    ) -> Result<R> {
        let response = self
            ._post(self.parameterized_uri(route, parameters)?, body)
            .await?;
        R::from_response(map_arcgis_error(response).await?).await
    }

    /// Send a `POST` request with no additional pre/post-processing.
    pub async fn _post<P: Serialize + ?Sized>(
        &self,
        url: impl TryInto<Url>,
        body: Option<&P>,
    ) -> Result<reqwest::Response> {
        let url = url
            .try_into()
            .map_err(|_| UriParseError {})
            .context(UriParseSnafu)?;
        let request = self.client.post(url);
        let request = self.build_request(request, body)?;
        self.execute(request).await
    }

    /// Send a `POST` request with multipart/form-data encoding
    pub async fn post_multipart<R: FromResponse>(
        &self,
        route: impl AsRef<str>,
        form: reqwest::multipart::Form,
    ) -> Result<R> {
        let url = self.parameterized_uri(route, None::<&()>)?;
        let response = self._post_multipart(url, form).await?;
        R::from_response(map_arcgis_error(response).await?).await
    }

    /// Send a `POST` request with multipart form data with no additional pre/post-processing.
    async fn _post_multipart(
        &self,
        url: impl TryInto<Url>,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::Response> {
        let url = url
            .try_into()
            .map_err(|_| UriParseError {})
            .context(UriParseSnafu)?;
        let request = self
            .client
            .post(url)
            .multipart(form)
            .build()
            .context(ReqwestSnafu)?;
        self.execute(request).await
    }

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

        let bytes = map_arcgis_error(response).await?;
        R::from_response(bytes).await
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
            // not sure if this should be application/json or www-form-urlencoded
            builder = builder.header(http::header::CONTENT_TYPE, "www-form-urlencoded");
            let body = serde_urlencoded::to_string(body).context(SerdeUrlEncodedSnafu)?;
            let request = builder.body(body).build().context(ReqwestSnafu)?;
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
        let (auth, cached_token) =
            if let AuthState::LegacyToken { auth, token, .. } = &self.auth_state {
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
            AuthState::LegacyToken {
                ref token,
                ref refresh_mutex,
                ..
            } => {
                // Fast path: check if we have a valid token
                let token = if let Some(token) = token.valid_token() {
                    token
                } else {
                    // Acquire mutex to ensure only one task refreshes at a time
                    let _guard = refresh_mutex.lock().await;

                    // Double-check: another task might have refreshed while we waited
                    if let Some(token) = token.valid_token() {
                        token
                    } else {
                        // Still need to refresh
                        self.refresh_token_legacy_token().await?
                    }
                };
                let mut header =
                    HeaderValue::from_str(format!("Bearer {}", token.expose_secret()).as_str())
                        .map_err(http::Error::from) // How does this work?
                        .context(HttpSnafu)?;
                header.set_sensitive(true);
                Some(header)
            }
            _ => None,
        };

        if let Some(mut auth_header) = auth_header {
            // Only set the auth_header if the authority (host) is to ArcGIS.
            // Otherwise, leave it off as we could have been redirected
            // away from ArcGIS (via follow_location_to_data()), and we don't
            // want to give our credentials to third-party services.

            if request.url().authority() == self.portal.authority() {
                auth_header.set_sensitive(true);
                request
                    .headers_mut()
                    .insert("X-Esri-Authorization", auth_header);
            }
        }

        // append f=json to the url
        request.url_mut().query_pairs_mut().append_pair("f", "json");

        // send request
        let response = self.client.execute(request).await.context(ReqwestSnafu)?;

        let status = response.status();
        if StatusCode::UNAUTHORIZED == status {
            if let AuthState::LegacyToken { token, .. } = &self.auth_state {
                token.clear();
            }
        }

        Ok(response)
    }
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
        self.auth = Auth::LegacyToken(LegacyToken::new(
            username.into(),
            password.into(),
            referer.into(),
            expiration.into(),
        ));
        self
    }

    pub fn build(self) -> ArcGISSharingClient {
        let auth = match self.auth {
            Auth::None => AuthState::None,
            Auth::LegacyToken(auth) => AuthState::LegacyToken {
                auth: auth.clone(),
                token: CachedToken::default(),
                refresh_mutex: Arc::new(tokio::sync::Mutex::new(())),
            },
        };

        // Ensure portal URL ends with a trailing slash for correct Url::join() behavior
        let mut portal_str = self.portal.expect("No portal provided");
        if !portal_str.ends_with('/') {
            portal_str.push('/');
        }
        let portal = Url::parse(&portal_str).expect("Invalid portal URL");

        ArcGISSharingClient {
            client: reqwest::Client::new(),
            portal,
            auth_state: auth,
        }
    }
}

/// # ArcGIS Sharing API Methods
impl ArcGISSharingClient {
    pub fn create_group(&self) -> CreateGroupHandler<'_> {
        CreateGroupHandler::new(self)
    }

    pub fn groups(&self, id: impl Into<String>) -> GroupsHandler<'_> {
        GroupsHandler::new(self, id.into())
    }

    pub fn search_groups(&self) -> GroupSearchBuilder<'_> {
        GroupSearchBuilder::new(self)
    }

    pub fn feature_service(&self, url: impl Into<String>) -> FeatureServiceHandler<'_> {
        FeatureServiceHandler::new(self, url.into())
    }

    pub fn content(&self, username: Option<impl Into<String>>) -> ContentHandler<'_> {
        // if username is provided, use it;
        // otherwise, use the username from the auth state
        let username = match username {
            Some(username) => Some(username.into()),
            None => match self.auth_state {
                AuthState::LegacyToken { ref auth, .. } => {
                    Some(auth.username.expose_secret().to_string())
                }
                _ => None,
            },
        }
        .expect("No username provided");

        ContentHandler::new(self, username)
    }

    pub fn item(
        &self,
        username: Option<impl Into<String>>,
        id: impl Into<String>,
    ) -> ItemHandler<'_> {
        // if username is provided, use it;
        // otherwise, use the username from the auth state
        let username = match username {
            Some(username) => Some(username.into()),
            None => match self.auth_state {
                AuthState::LegacyToken { ref auth, .. } => {
                    Some(auth.username.expose_secret().to_string())
                }
                _ => None,
            },
        }
        .expect("No username provided");

        ItemHandler::new(self, username, id.into())
    }

    pub fn search(&self) -> SearchBuilder<'_> {
        SearchBuilder::new(self)
    }

    pub fn portals(&self) -> PortalsHandler<'_> {
        PortalsHandler::new(self)
    }
}
