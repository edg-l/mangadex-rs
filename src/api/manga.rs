use std::collections::HashMap;

use crate::errors::Result;
use crate::model::manga::*;
use crate::model::ApiData;
use crate::model::NoData;
use crate::model::PaginationQuery;
use crate::Client;
use serde::Serialize;
use uuid::Uuid;

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
pub struct ListFollowedManga<'a> {
    #[serde(flatten)]
    pub query: &'a PaginationQuery,
}

impl_endpoint! {
    GET "/user/follows/manga",
    #[query auth] ListFollowedManga<'_>,
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
pub struct ListTags;

impl_endpoint! {
    GET "/manga/tag",
    #[no_data] ListTags,
    TagList
}

/// Get logged user followed manga feed (requires authentication)
///
/// Returns the feed (chapters list) for the logged user
///
/// Call to `GET /user/follows/manga/feed`
#[derive(Debug, Serialize, Clone)]
pub struct FollowedMangaFeed<'a> {
    #[serde(flatten)]
    pub query: &'a MangaFeedQuery,
}

impl_endpoint! {
    GET "/user/follows/manga/feed",
    #[body auth] FollowedMangaFeed<'_>,
    ChapterList
}

/// Manga feed
///
/// Returns the feed (chapters list) for the specified manga
///
/// Call to `GET /manga/{id}/feed`
#[derive(Debug, Serialize, Clone)]
pub struct MangaFeed<'a> {
    #[serde(skip)]
    pub manga_id: &'a Uuid,
    #[serde(flatten)]
    pub query: &'a MangaFeedQuery,
}

impl_endpoint! {
    GET ("/manga/{:x}/feed", manga_id),
    #[query] MangaFeed<'_>,
    ChapterList
}

/// Get manga reading status (requires authentication)
///
/// Call to `GET /manga/{id}/status`
#[derive(Debug, Clone)]
pub struct GetMangaStatus<'a> {
    pub id: &'a Uuid,
}

impl_endpoint! {
    GET ("/manga/{:x}/status", id),
    #[no_data auth] GetMangaStatus<'_>,
    #[no_send] Result<MangaReadingStatusBody>
}

impl GetMangaStatus<'_> {
    pub async fn send(&self, client: &Client) -> Result<MangaReadingStatus> {
        client.send_request(self).await?.map(|r| r.status)
    }
}

/// Update manga reading status (requires authentication)
///
/// Call to `POST /manga/{id}/status`
#[derive(Debug, Serialize, Clone)]
pub struct UpdateMangaStatus<'a> {
    #[serde(skip)]
    pub id: &'a Uuid,
    pub status: MangaReadingStatus,
}

impl_endpoint! {
    POST ("/manga/{:x}/status", id),
    #[body auth] UpdateMangaStatus<'_>,
    #[discard_result] Result<NoData>
}

/// Get all manga reading status for logged user (requires authentication)
///
/// Call to `GET /manga/status`
#[derive(Serialize)]
pub struct AllMangaStatus {
    pub status: Option<MangaReadingStatus>,
}

impl_endpoint! {
    GET "/manga/status",
    #[query auth] AllMangaStatus,
    #[no_send] Result<MangaReadingStatuses>
}

impl AllMangaStatus {
    pub async fn send(&self, client: &Client) -> Result<HashMap<Uuid, MangaReadingStatus>> {
        client.send_request(self).await?.map(|r| r.statuses)
    }
}

/// Manga read markers (requires authentication)
///
/// A list of chapter ids that are marked as read for the specified manga.
///
/// Call to `GET /manga/read`
#[derive(Serialize)]
pub struct MangaReadMarkers<'a> {
    pub ids: Vec<&'a Uuid>,
    // pub grouped: Option<bool>,
}

impl_endpoint! {
    GET "/manga/read",
    #[query auth] MangaReadMarkers<'_>,
    #[flatten_result] Result<ApiData<Vec<Uuid>>>
}

#[cfg(test)]
mod tests {

    use crate::{
        model::{LanguageCode, ResourceType},
        Client,
    };

    use super::*;
    use chrono::prelude::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn list_manga() {
        let client = Client::default();
        let query = MangaQuery::default();
        let manga = ListManga { query: &query }.send(&client).await.unwrap();
        assert_eq!(manga.offset, 0);
        assert_eq!(manga.limit, 10);
    }

    #[tokio::test]
    async fn view_manga() {
        let id = Uuid::parse_str("32d76d19-8a05-4db0-9fc2-e0b0648fe9d0").unwrap();
        let client = Client::default();
        let manga_result = ViewManga { id: &id }.send(&client).await.unwrap();

        let manga = manga_result.data;
        assert_eq!(manga.id, id);
        assert_eq!(
            manga
                .attributes
                .title
                .get(&LanguageCode::English)
                .map(String::as_str),
            Some("Solo Leveling")
        );
        assert_eq!(manga.attributes.original_language.as_str(), "ko");
        // 2019-08-25T10:51:55+00:00
        assert_eq!(
            manga.attributes.created_at,
            Utc.ymd(2019, 8, 25).and_hms(10, 51, 55)
        );
    }

    #[tokio::test]
    async fn random_manga() {
        let client = Client::default();
        let manga_result = RandomManga.send(&client).await.unwrap();
        let manga = manga_result.data;
        assert_eq!(manga.r#type, ResourceType::Manga);
    }

    #[tokio::test]
    async fn tag_list() {
        let client = Client::default();
        let tag_results = ListTags.send(&client).await.unwrap();

        for result in &tag_results {
            let tag = &result.as_ref().unwrap().data;
            assert_eq!(tag.r#type, ResourceType::Tag);
        }
    }
}
