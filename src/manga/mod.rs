use crate::{errors::Result, NoData, PaginationQuery};

use serde::Serialize;
use uuid::Uuid;

mod tests;

mod types;
pub use types::*;

mod feed;
pub use feed::*;

mod status;
pub use status::*;

/// Manga list
///
/// Search a list of manga
///
/// Call to `GET /manga`
#[derive(Debug, Serialize, Clone)]
pub struct ListManga<'a> {
    #[serde(flatten)]
    pub query: &'a MangaQuery,
}

impl_endpoint! {
    GET "/manga",
    #[query] ListManga<'_>,
    MangaList
}

/// Create manga (requires authentication)
///
/// Create a new manga
///
/// Call to `POST /manga`
#[derive(Debug, Serialize, Clone)]
pub struct CreateManga<'a> {
    #[serde(flatten)]
    pub request: &'a MangaRequest,
}

impl_endpoint! {
    POST "/manga",
    #[body auth] CreateManga<'_>,
    #[flatten_result] MangaResponse
}

/// Update manga (requires authentication)
///
/// Update an existing manga
///
/// Call to `PUT /manga/{id}`
#[derive(Debug, Serialize, Clone)]
pub struct UpdateManga<'a> {
    #[serde(skip)]
    pub id: &'a Uuid,
    #[serde(flatten)]
    pub request: &'a MangaRequest,
}

impl_endpoint! {
    PUT ("/manga/{:x}", id),
    #[body auth] UpdateManga<'_>,
    #[flatten_result] MangaResponse
}

/// View manga
///
/// Call to `GET /manga/{id}`
#[derive(Debug, Clone)]
pub struct ViewManga<'a> {
    pub id: &'a Uuid,
}

impl_endpoint! {
    GET ("/manga/{:x}", id),
    #[no_data] ViewManga<'_>,
    #[flatten_result] MangaResponse
}

/// Delete manga (requires authentication)
///
/// Call to `DELETE /manga/{id}`
#[derive(Debug, Clone)]
pub struct DeleteManga<'a> {
    pub id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/manga/{:x}", id),
    #[no_data auth] DeleteManga<'_>,
    #[discard_result] Result<NoData>
}

/// Follow manga (requires authentication)
///
/// Call to `POST /manga/{id}/follow`
#[derive(Debug, Clone)]
pub struct FollowManga<'a> {
    pub id: &'a Uuid,
}

impl_endpoint! {
    POST ("/manga/{:x}/follow", id),
    #[no_data auth] FollowManga<'_>,
    #[discard_result] Result<NoData>
}

/// Unfollow manga (requires authentication)
///
/// Call to `DELETE /manga/{id}/follow`
pub struct UnfollowManga<'a> {
    pub id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/manga/{:x}/follow", id),
    #[no_data auth] UnfollowManga<'_>,
    #[discard_result] Result<NoData>
}

/// Get logged user followed manga list (requires authentication)
///
/// Call to `GET /usr/follows/manga`
#[derive(Debug, Serialize, Clone)]
pub struct FollowedMangaList<'a> {
    #[serde(flatten)]
    pub query: &'a PaginationQuery,
}

impl_endpoint! {
    GET "/user/follows/manga",
    #[query auth] FollowedMangaList<'_>,
    MangaList
}

/// Get a random manga
///
/// Call to `GET /manga/random`
pub struct RandomManga;

impl_endpoint! {
    GET "/manga/random",
    #[no_data] RandomManga,
    #[flatten_result] MangaResponse
}

/// Global tag list
///
/// Call to `GET /manga/tags`
pub struct TagList;

impl_endpoint! {
    GET "/manga/tag",
    #[no_data] TagList,
    types::TagList
}
