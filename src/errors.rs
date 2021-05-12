use thiserror::Error;

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
}

/// Helper Result type.
pub type Result<T, E = Errors> = std::result::Result<T, E>;
