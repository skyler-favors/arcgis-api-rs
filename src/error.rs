use http::uri::InvalidUri;
use snafu::Snafu;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

#[cfg(feature = "error-backtrace")]
use snafu::Backtrace;

//This is workaround until I figure out how to get TryInto errors to work
#[derive(Debug)]
pub struct UriParseError;

impl Display for UriParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse URI")
    }
}

impl std::error::Error for UriParseError {}

/// An error that could have occurred while using [`crate::ArcGISSharingClient`].
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum Error {
    #[snafu(display("ArcGIS Error: {}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    Arcgis { source: Box<ArcgisError> },

    #[snafu(display("ArcGIS Error: {}\nFound at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    Arcgis {
        source: Box<ArcgisError>,
        backtrace: Backtrace,
    },

    #[snafu(display("URL Parse Error: {}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    UrlParse { source: url::ParseError },

    #[snafu(display("URL Parse Error: {} at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    UrlParse {
        source: url::ParseError,
        backtrace: Backtrace,
    },

    #[snafu(display("Invalid Header: {}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    UriParse { source: UriParseError },

    #[snafu(display("Invalid Header: {} at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    UriParse {
        source: UriParseError,
        backtrace: Backtrace,
    },

    #[snafu(display("URL Parse Error: {}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    Uri { source: InvalidUri },

    #[snafu(display("URL Parse Error: {} at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    Uri {
        source: InvalidUri,
        backtrace: Backtrace,
    },

    #[snafu(display("LegacyAuth error"))]
    #[cfg(not(feature = "error-backtrace"))]
    LegacyAuth,

    #[snafu(display("LegacyAuth error.\n\nFound at {}", backtrace))]
    #[cfg(feature = "error-backtrace")]
    LegacyAuth { backtrace: Backtrace },

    #[snafu(display("{}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    InvalidHeaderValue {
        source: http::header::InvalidHeaderValue,
    },

    #[snafu(display("{}\n\nFound at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    InvalidHeaderValue {
        source: http::header::InvalidHeaderValue,
        backtrace: Backtrace,
    },

    #[snafu(display("Reqwest Error: {}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    Reqwest { source: reqwest::Error },

    #[snafu(display("Reqwest Error: {}\n\nFound at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    Reqwest {
        source: reqwest::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("HTTP Error: {}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    Http { source: http::Error },

    #[snafu(display("HTTP Error: {}\n\nFound at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    Http {
        source: http::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("{}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    InvalidUtf8 { source: FromUtf8Error },

    #[snafu(display("{}\n\nFound at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    InvalidUtf8 {
        source: FromUtf8Error,
        backtrace: Backtrace,
    },

    #[snafu(display("{}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    Encoder { source: std::io::Error },

    #[snafu(display("{}\n\nFound at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    Encoder {
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Serde Url Encode Error: {}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    SerdeUrlEncoded {
        source: serde_urlencoded::ser::Error,
    },

    #[snafu(display("Serde Url Encode Error: {}\nFound at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    SerdeUrlEncoded {
        source: serde_urlencoded::ser::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Serde Error: {}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    Serde { source: serde_json::Error },

    #[snafu(display("Serde Error: {}\nFound at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    Serde {
        source: serde_json::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("JSON Error in {}: {}", source.path(), source.inner()))]
    #[cfg(not(feature = "error-backtrace"))]
    Json {
        source: serde_path_to_error::Error<serde_json::Error>,
    },

    #[snafu(display(
        "JSON Error in {}: {}\nFound at {}",
        source.path(),
        source.inner(),
        backtrace
    ))]
    #[cfg(feature = "error-backtrace")]
    Json {
        source: serde_path_to_error::Error<serde_json::Error>,
        backtrace: Backtrace,
    },

    #[snafu(display("{}", source))]
    #[cfg(not(feature = "error-backtrace"))]
    Other {
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[snafu(display("{}\n\nFound at {}", source, backtrace))]
    #[cfg(feature = "error-backtrace")]
    Other {
        source: Box<dyn std::error::Error + Send + Sync>,
        backtrace: Backtrace,
    },
}

impl Error {
    pub(crate) fn arcgis(source: ArcgisError) -> Self {
        Error::Arcgis {
            source: Box::new(source),
            #[cfg(feature = "error-backtrace")]
            backtrace: Backtrace::capture(),
        }
    }

    pub(crate) fn legacy_auth() -> Self {
        #[cfg(feature = "error-backtrace")]
        {
            Error::LegacyAuth {
                backtrace: Backtrace::capture(),
            }
        }
        #[cfg(not(feature = "error-backtrace"))]
        {
            Error::LegacyAuth
        }
    }

    pub(crate) fn json(source: serde_path_to_error::Error<serde_json::Error>) -> Self {
        Error::Json {
            source,
            #[cfg(feature = "error-backtrace")]
            backtrace: Backtrace::capture(),
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ArcgisError {
    pub code: i32,
    pub message_code: Option<String>,
    pub message: String,
    pub details: Option<Vec<String>>,
}

impl fmt::Display for ArcgisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format: "400 [CODE] - Message"
        write!(f, "{}", self.code)?;

        if let Some(ref code) = self.message_code {
            write!(f, " / {code}")?;
        }

        write!(f, " - {}", self.message)?;

        if let Some(ref errors) = self.details {
            for error in errors {
                write!(f, "\n  Detail: {error}")?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for ArcgisError {}

pub type Result<T, E = Error> = std::result::Result<T, E>;
