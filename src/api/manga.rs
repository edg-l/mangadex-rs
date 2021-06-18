use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::model::{manga::*, NoData, PaginationQuery};
use crate::{Client, Result};

/// Manga list
///
/// Search a list of manga
///
/// Call to `GET /manga`
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ListManga<'a> {
    /// Page size
    pub limit: Option<i32>,

    /// Page offset
    pub offset: Option<i32>,

    /// Manga title
    pub title: Option<String>,

    /// Manga authors
    #[builder(setter(each = "add_author"))]
    pub authors: Vec<&'a Uuid>,

    /// Manga artists
    #[builder(setter(each = "add_artist"))]
    pub artists: Vec<&'a Uuid>,

    /// Year of release
    pub year: Option<i32>,

    /// Included tags
    #[builder(setter(each = "include_tag"))]
    pub included_tags: Vec<&'a Uuid>,

    /// Tag inclusion mode
    pub included_tags_mode: Option<TagMode>,

    /// Excluded tags
    #[builder(setter(each = "exclude_tag"))]
    pub excluded_tags: Vec<&'a Uuid>,

    /// Tag exclusion mode
    pub excluded_tags_mode: Option<TagMode>,

    /// Manga status
    #[builder(setter(each = "add_status"))]
    pub status: Vec<MangaStatus>,

    /// Original language
    pub original_language: Vec<String>,

    /// Publication demographic
    #[builder(setter(each = "add_demographic"))]
    pub publication_demographic: Vec<Demographic>,

    /// Manga ids
    #[builder(setter(each = "add_manga"))]
    #[serde(rename = "ids")]
    pub manga_ids: Vec<Uuid>,

    /// Content rating
    pub content_rating: Option<ContentRating>,

    /// Created after this time
    pub created_at_since: Option<DateTime<Utc>>,

    /// Updated after this time
    pub updated_at_since: Option<DateTime<Utc>>,

    /// Sorting order
    pub order: Option<MangaOrder>,
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

/// View manga
///
/// Call to `GET /manga/{id}`
#[derive(Debug, Clone)]
pub struct GetManga<'a> {
    pub id: &'a Uuid,
}

impl_endpoint! {
    GET ("/manga/{:x}", id),
    #[no_data] GetManga<'_>,
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

/// Manga read markers (requires authentication)
///
/// A list of chapter ids that are marked as read for the specified manga
///
/// Call to `GET /manga/{manga_id}/read`
#[derive(Debug, Clone)]
pub struct GetMangaReadMarkers<'a> {
    /// Manga id
    pub manga_id: &'a Uuid,
}

impl_endpoint! {
    GET ("/manga/{:x}/read", manga_id),
    #[no_data auth] GetMangaReadMarkers<'_>,
    #[flatten_result] MangaReadMarkerResponse
}

/// Manga read markers (requires authentication)
///
/// A list of chapter ids that are marked as read for the given manga ids
///
/// Call to `GET /manga/read`
#[derive(Serialize)]
pub struct GetBatchMangaReadMarkers<'a> {
    pub ids: Vec<&'a Uuid>,
    // pub grouped: Option<bool>,
}

impl_endpoint! {
    GET "/manga/read",
    #[query auth] GetBatchMangaReadMarkers<'_>,
    #[flatten_result] MangaReadMarkerResponse
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
    #[flatten_result] Result<MangaReadingStatuses>
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
        let manga = ListManga::default().send(&client).await.unwrap();
        assert_eq!(manga.offset, 0);
        assert_eq!(manga.limit, 10);
    }

    #[tokio::test]
    async fn view_manga() {
        let id = Uuid::parse_str("32d76d19-8a05-4db0-9fc2-e0b0648fe9d0").unwrap();
        let client = Client::default();
        let manga_result = GetManga { id: &id }.send(&client).await.unwrap();

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
