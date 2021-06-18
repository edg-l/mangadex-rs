//! Manga authors

use derive_builder::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::model::author::*;
use crate::model::NoData;
use crate::model::PaginationQuery;
use crate::Result;

/// List authors
///
/// Call to `GET /author`
#[skip_serializing_none]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option), default)]
pub struct ListAuthors<'a> {
    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationQuery,

    /// Author ids (limited to 100 per request)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_id"))]
    pub ids: Vec<&'a Uuid>,

    /// Author name
    pub name: Option<&'a str>,

    /// Result order
    pub order: Option<AuthorOrder>,
}

impl_endpoint! {
    GET "/author",
    #[query] ListAuthors<'_>,
    AuthorList
}

/// Create a new author (requires authentication)
///
/// Call to `POST /author`
#[derive(Debug, Serialize, Clone)]
pub struct CreateAuthor<'a> {
    /// Name of the author
    pub name: &'a str,

    /// Version (minimum: 1)
    pub version: i32,
}

impl_endpoint! {
    POST "/author",
    #[body auth] CreateAuthor<'_>,
    #[flatten_result] AuthorResponse
}

/// Get author information
///
/// Call to `GET /author/{id}`
#[derive(Debug, Clone)]
pub struct GetAuthor<'a> {
    /// Author id
    pub id: &'a Uuid,
}

impl_endpoint! {
    GET ("/author/{:x}", id),
    #[no_data] GetAuthor<'_>,
    #[flatten_result] AuthorResponse
}

/// Update an existing author (requires authentication)
///
/// Call to `PUT /author/{id}`
#[derive(Debug, Serialize, Clone)]
pub struct UpdateAuthor<'a> {
    /// Author id
    #[serde(skip)]
    pub id: &'a Uuid,

    /// Name of the author
    pub name: &'a str,

    /// Version (minimum: 1)
    pub version: i32,
}

impl_endpoint! {
    PUT ("/author/{:x}", id),
    #[body auth] UpdateAuthor<'_>,
    #[flatten_result] AuthorResponse
}

/// Delete author (requires authentication)
///
/// Call to `DELETE /author/{id}`
#[derive(Debug, Clone)]
pub struct DeleteAuthor<'a> {
    /// Author id
    pub id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/author/{:x}", id),
    #[no_data auth] DeleteAuthor<'_>,
    #[discard_result] Result<NoData>
}
