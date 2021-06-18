use thiserror::Error;

use crate::schema::errors::ApiErrors;

/// A enum with all the possible errors
#[derive(Debug, Error)]
pub enum Errors {
    /// Error when parsing a url
    /// Shouldn't really happen at all
    #[error("parse url error")]
    ParseUrl(#[from] url::ParseError),

    /// Possible unhandled http errors by the wrapper that may arise when calling the api
    #[error("http error")]
    Http(#[from] reqwest::Error),

    /// Missing client tokens
    #[error("missing tokens error")]
    MissingTokens,

    /// Api server returned an error
    #[error("api error")]
    Api(#[from] ApiErrors),

    /// Received an unexpected response from /ping
    #[error("invalid ping response")]
    PingError,
}

/// Helper Result type.
pub type Result<T, E = Errors> = std::result::Result<T, E>;
