//! Chapter data

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::{chapter::*, LanguageCode, NoData};
use crate::Result;

/// Chapter list
///
/// Call to `GET /chapter`
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option), default)]
pub struct ListChapters<'a> {
    /// Page size
    pub limit: Option<i32>,

    /// Page offset
    pub offset: Option<i32>,

    /// Chapter ids
    #[serde(rename = "ids")]
    #[builder(setter(each = "add_chapter"))]
    pub chapter_ids: Vec<&'a Uuid>,

    /// Chapter title
    pub title: Option<&'a str>,

    /// Groups
    #[builder(setter(each = "add_group"))]
    pub groups: Vec<&'a Uuid>,

    /// Uploader
    pub uploader: Option<&'a Uuid>,

    /// Manga
    pub manga: Option<&'a Uuid>,

    /// Volume
    pub volume: Option<&'a str>,

    /// Chapter
    pub chapter: Option<&'a str>,

    /// Translated language
    pub translated_language: Option<LanguageCode>,

    /// Created after
    pub created_at_since: Option<DateTime<Utc>>,

    /// Updated after
    pub updated_at_since: Option<DateTime<Utc>>,

    /// Published after
    pub publish_at_since: Option<DateTime<Utc>>,

    /// Sort order
    pub order: Option<ChapterOrder>,
}

impl_endpoint! {
    GET "/chapter",
    #[query] ListChapters<'_>,
    ChapterList
}

/// Get chapter
///
/// Call to `GET /chapter/{chapter_id}`
#[derive(Debug, Clone)]
pub struct GetChapter<'a> {
    /// Chapter id
    pub chapter_id: &'a Uuid,
}

impl_endpoint! {
    GET ("/chapter/{:x}", chapter_id),
    #[no_data] GetChapter<'_>,
    #[flatten_result] ChapterResponse
}

/// Update chapter (requires authentication)
///
/// Call to `PUT /chapter/{chapter_id}
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChapter<'a> {
    /// Chapter id
    #[serde(skip)]
    pub chapter_id: &'a Uuid,

    /// Chapter title (max length: 255)
    pub title: &'a str,

    /// Chapter volume
    pub volume: Option<&'a str>,

    /// Chapter number (max length: 8)
    pub chapter: Option<&'a str>,

    /// Translated language
    pub translated_language: LanguageCode,

    /// Data quality image names
    pub data: Vec<&'a str>,

    /// Data-saver quality image names
    pub data_saver: Vec<&'a str>,

    /// Chapter version (min: 1)
    pub version: i32,
}

impl_endpoint! {
    PUT ("/chapter/{:x}", chapter_id),
    #[body auth] UpdateChapter<'_>,
    #[flatten_result] ChapterResponse
}

/// Delete a chapter (requires authorization)
///
/// Call to `DELETE /chapter/{id}`
#[derive(Debug, Clone)]
pub struct DeleteChapter<'a> {
    chapter_id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/chapter/{:x}", chapter_id),
    #[no_data auth] DeleteChapter<'_>,
    #[discard_result] Result<NoData>
}

/// Mark chapter as read (requires authorization)
///
/// Call to `POST /chapter/{id}/read`
#[derive(Debug, Clone)]
pub struct MarkChapterRead<'a> {
    chapter_id: &'a Uuid,
}

impl_endpoint! {
    POST ("/chapter/{:x}/read", chapter_id),
    #[no_data auth] MarkChapterRead<'_>,
    #[discard_result] Result<NoData>
}

/// Mark chapter as unread (requires authorization)
///
/// Call to `DELETE /chapter/{id}/read`
#[derive(Debug, Clone)]
pub struct MarkChapterUnread<'a> {
    chapter_id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/chapter/{:x}/read", chapter_id),
    #[no_data auth] MarkChapterUnread<'_>,
    #[discard_result] Result<NoData>
}
