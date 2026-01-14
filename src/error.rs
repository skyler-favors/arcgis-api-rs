use http::uri::InvalidUri;
use snafu::{Backtrace, Snafu};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

//This is workaround until I figure out how to get TryInto errors to work
#[derive(Debug)]
pub struct UriParseError;

impl Display for UriParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse URI")
    }
}

impl std::error::Error for UriParseError {}

/// An error that could have occurred while using [`crate::Octocrab`].
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum Error {
    Arcgis {
        source: Box<ArcgisError>,
        backtrace: Backtrace,
    },
    UrlParse {
        source: url::ParseError,
        backtrace: Backtrace,
    },

    UriParse {
        source: UriParseError,
        backtrace: Backtrace,
    },
    Uri {
        source: InvalidUri,
        backtrace: Backtrace,
    },
    #[snafu(display("LegacyAuth error.\n\nFound at {}", backtrace))]
    LegacyAuth { backtrace: Backtrace },

    InvalidHeaderValue {
        source: http::header::InvalidHeaderValue,
        backtrace: Backtrace,
    },

    #[snafu(display("Reqwest Error: {}\n\nFound at {}", source, backtrace))]
    Reqwest {
        source: reqwest::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("HTTP Error: {}\n\nFound at {}", source, backtrace))]
    Http {
        source: http::Error,
        backtrace: Backtrace,
    },

    InvalidUtf8 {
        source: FromUtf8Error,
        backtrace: Backtrace,
    },

    Encoder {
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Serde Url Encode Error: {}\nFound at {}", source, backtrace))]
    SerdeUrlEncoded {
        source: serde_urlencoded::ser::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Serde Error: {}\nFound at {}", source, backtrace))]
    Serde {
        source: serde_json::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("JSON Error in {}: {}\nFound at {}", source.path(), source.inner(), backtrace))]
    Json {
        source: serde_path_to_error::Error<serde_json::Error>,
        backtrace: Backtrace,
    },
    // #[snafu(display("JWT Error in {}\nFound at {}", source, backtrace))]
    // JWT {
    //     source: jsonwebtoken::errors::Error,
    //     backtrace: Backtrace,
    // },
    Other {
        source: Box<dyn std::error::Error + Send + Sync>,
        backtrace: Backtrace,
    },
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
        write!(f, "{}", self.code)?;

        if let Some(code) = self.message_code.as_ref() {
            write!(f, " / {code}")?;
        }

        write!(f, " - {}", self.message)?;

        if let Some(errors) = &self.details {
            write!(f, "\nErrors:")?;
            for error in errors.iter() {
                write!(f, "\n- {error}")?;
            }
        }

        Ok(())
    }
}
impl std::error::Error for ArcgisError {}

pub type Result<T, E = Error> = std::result::Result<T, E>;
