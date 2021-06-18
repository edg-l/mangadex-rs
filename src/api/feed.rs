//! Chapter feed

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::model::{feed::*, manga::ChapterList, LanguageCode};

/// Manga feed
///
/// Returns the feed (chapters list) for the specified manga
///
/// Call to `GET /manga/{id}/feed`
#[derive(Debug, Serialize, Clone, Builder)]
#[builder(setter(strip_option))]
#[serde(rename_all = "camelCase")]
pub struct GetMangaFeed<'a> {
    /// Manga id
    #[serde(skip)]
    pub manga_id: &'a Uuid,

    /// Page size
    pub limit: Option<i32>,

    /// Page offset
    pub offset: Option<i32>,

    /// Translated language
    #[builder(default)]
    #[builder(setter(each = "add_language"))]
    pub translated_language: Vec<LanguageCode>,

    /// Created after
    #[builder(default)]
    pub created_at_since: Option<DateTime<Utc>>,

    /// Updated after
    #[builder(default)]
    pub updated_at_since: Option<DateTime<Utc>>,

    /// Published after
    #[builder(default)]
    pub publish_at_since: Option<DateTime<Utc>>,

    /// Sort order
    #[builder(default)]
    pub order: Option<FeedOrder>,
}

impl_endpoint! {
    GET ("/manga/{:x}/feed", manga_id),
    #[query] GetMangaFeed<'_>,
    ChapterList
}

/// Get logged user followed manga feed (requires authentication)
///
/// Returns the feed (chapters list) for the logged user
///
/// Call to `GET /user/follows/manga/feed`
#[derive(Debug, Serialize, Clone, Builder)]
#[builder(setter(strip_option))]
#[serde(rename_all = "camelCase")]
pub struct FollowedMangaFeed {
    /// Page size
    pub limit: Option<i32>,

    /// Page offset
    pub offset: Option<i32>,

    /// Translated language
    #[builder(default)]
    #[builder(setter(each = "add_language"))]
    pub translated_language: Vec<LanguageCode>,

    /// Created after
    #[builder(default)]
    pub created_at_since: Option<DateTime<Utc>>,

    /// Updated after
    #[builder(default)]
    pub updated_at_since: Option<DateTime<Utc>>,

    /// Published after
    #[builder(default)]
    pub publish_at_since: Option<DateTime<Utc>>,

    /// Sort order
    #[builder(default)]
    pub order: Option<FeedOrder>,
}

impl_endpoint! {
    GET "/user/follows/manga/feed",
    #[body auth] FollowedMangaFeed,
    ChapterList
}

/// Custom list manga feed
///
/// Call to `GET /list/{list_id}/feed`
#[derive(Debug, Serialize, Clone, Builder)]
#[builder(setter(strip_option))]
#[serde(rename_all = "camelCase")]
pub struct CustomListMangaFeed<'a> {
    /// List id
    #[serde(skip)]
    pub list_id: &'a Uuid,

    /// Page size
    pub limit: Option<i32>,

    /// Page offset
    pub offset: Option<i32>,

    /// Translated language
    #[builder(default)]
    #[builder(setter(each = "add_language"))]
    pub translated_language: Vec<LanguageCode>,

    /// Created after
    #[builder(default)]
    pub created_at_since: Option<DateTime<Utc>>,

    /// Updated after
    #[builder(default)]
    pub updated_at_since: Option<DateTime<Utc>>,

    /// Published after
    #[builder(default)]
    pub publish_at_since: Option<DateTime<Utc>>,

    /// Sort order
    #[builder(default)]
    pub order: Option<FeedOrder>,
}

impl_endpoint! {
    GET ("/list/{:x}/feed", list_id),
    #[query] CustomListMangaFeed<'_>,
    ChapterList
}
