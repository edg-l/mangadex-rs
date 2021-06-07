use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;

/// A enum with all the possible errors.
#[derive(Debug, Error)]
pub enum Errors {
    /// Error when parsing a url.
    /// Shouldn't really happen at all.
    #[error("parse url error")]
    ParseUrl(#[from] url::ParseError),
    /// Possible unhandled http errors by the wrapper that may arise when calling the api.
    #[error("http error")]
    Http(#[from] reqwest::Error),
    #[error("missing tokens error")]
    MissingTokens,
    #[error("http error with body")]
    HttpWithBody(#[from] ApiErrors),
    /// Received an unexpected response from /ping.
    #[error("invalid ping response")]
    PingError,
}

#[derive(Debug, Error, Deserialize)]
#[error("bad request")]
pub struct ApiErrors {
    /// A list of errors.
    #[serde(default)]
    pub errors: Vec<ApiError>,
}

#[derive(Debug, Error, PartialEq, Eq, Deserialize)]
#[error("api error")]
pub struct ApiError {
    /// The error id.
    pub id: Uuid,
    /// The error status.
    pub status: i32,
    /// The error title.
    pub title: Option<String>,
    /// Details about the error.
    pub detail: Option<String>,
}

/// Helper Result type.
pub type Result<T, E = Errors> = std::result::Result<T, E>;
