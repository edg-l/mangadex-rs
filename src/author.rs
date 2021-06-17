use crate::{
    common::{ApiObject, Results},
    errors::Result,
    ApiData, NoData, OrderType, PaginationQuery,
};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorOrder {
    pub name: OrderType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorAttributes {
    pub name: String,
    pub image_url: Option<String>,
    // pub biography: HashMap<String, String>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type Author = ApiObject<AuthorAttributes>;
pub type AuthorResponse = Result<ApiData<Author>>;
pub type AuthorList = Results<AuthorResponse>;

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ListAuthorsReq {
    #[serde(flatten)]
    pub pagination: PaginationQuery,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_id"))]
    pub ids: Vec<Uuid>,

    pub name: Option<String>,
    pub order: Option<AuthorOrder>,
}

impl_endpoint! {
    GET "/author",
    #[query] ListAuthorsReq,
    AuthorList
}

/// Create author (requires authentication)
///
/// Call to `POST /author`
#[derive(Debug, Serialize, Clone)]
pub struct CreateAuthorReq<'a> {
    name: &'a str,
    version: i32,
}

impl_endpoint! {
    POST "/author",
    #[body auth] CreateAuthorReq<'_>,
    #[flatten_result] AuthorResponse
}

/// Update author (requires authentication)
///
/// Call to `POST /author/{id}`
#[derive(Debug, Serialize, Clone)]
pub struct UpdateAuthorReq<'a> {
    #[serde(skip)]
    id: &'a Uuid,
    name: &'a str,
    version: i32,
}

impl_endpoint! {
    PUT ("/author/{:x}", id),
    #[body auth] UpdateAuthorReq<'_>,
    #[flatten_result] AuthorResponse
}

/// Get author
///
/// Call to `GET /author/{id}`
#[derive(Debug, Clone)]
pub struct GetAuthorReq<'a> {
    id: &'a Uuid,
}

impl_endpoint! {
    GET ("/author/{:x}", id),
    #[no_data] GetAuthorReq<'_>,
    #[flatten_result] AuthorResponse
}

/// Delete author (requires authentication)
///
/// Call to `DELETE /author/{id}`
#[derive(Debug, Clone)]
pub struct DeleteAuthorReq<'a> {
    id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/author/{:x}", id),
    #[no_data auth] DeleteAuthorReq<'_>,
    #[discard_result] Result<NoData>
}
