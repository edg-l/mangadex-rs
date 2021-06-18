use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;

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
