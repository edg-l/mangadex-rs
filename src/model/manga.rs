use crate::{common::deserialize_null_default, errors::Result};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use uuid::Uuid;

use super::{ApiData, ApiObject, LocalizedString, OrderType, PaginationQuery, Results};

/// The tag mode.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TagMode {
    // AND Mode
    And,
    // OR Mode
    Or,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TagAttributes {
    pub name: LocalizedString,
    // TODO: Known issue: empty descriptions return [] instead of {}
    #[serde(skip)]
    pub description: LocalizedString,
    pub group: String,
    pub version: i32,
}

/// The status of a manga.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MangaStatus {
    Ongoing,
    Completed,
    Hiatus,
    Cancelled,
}

/// The publication demographic.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Demographic {
    Shounen,
    Shoujo,
    Josei,
    Seinen,
    None,
}

/// The content rating of the publication.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContentRating {
    Safe,
    Suggestive,
    Erotica,
    Pornographic,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub created_at: OrderType,
    pub updated_at: OrderType,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeedOrder {
    pub volume: OrderType,
    pub chapter: OrderType,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct MangaQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,

    pub title: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_author"))]
    pub authors: Vec<Uuid>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_artist"))]
    pub artists: Vec<Uuid>,

    pub year: Option<i32>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_tag"))]
    pub included_tags: Vec<Uuid>,

    pub included_tags_mode: Option<TagMode>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_status"))]
    pub status: Vec<MangaStatus>,

    pub original_language: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_demographic"))]
    pub publication_demographic: Vec<Demographic>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_id"))]
    pub ids: Vec<Uuid>,

    pub content_rating: Option<ContentRating>,

    pub created_at_since: Option<DateTime<Utc>>,

    pub updated_at_since: Option<DateTime<Utc>>,

    pub order: Option<Order>,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize, Clone, PartialEq, Eq)]
#[builder(setter(strip_option))]
#[serde(rename_all = "camelCase")]
pub struct MangaFeedQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translated_language: Vec<String>,

    #[builder(default)]
    pub created_at_since: Option<DateTime<Utc>>,

    #[builder(default)]
    pub updated_at_since: Option<DateTime<Utc>>,

    #[builder(default)]
    pub publish_at_since: Option<DateTime<Utc>>,

    #[builder(default)]
    pub order: Option<FeedOrder>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Links {
    pub al: Option<String>,
    pub ap: Option<String>,
    pub bw: Option<String>,
    pub mu: Option<String>,
    pub nu: Option<String>,
    pub kt: Option<String>,
    pub amz: Option<String>,
    pub ebj: Option<String>,
    pub mal: Option<String>,
    pub raw: Option<String>,
    pub engtl: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MangaAttributes {
    pub title: LocalizedString,
    pub alt_titles: Vec<LocalizedString>,
    // TODO: Known issue: empty descriptions return [] instead of {}
    #[serde(skip)]
    pub description: LocalizedString,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub links: Links,
    pub original_language: String,
    pub last_volume: Option<String>,
    pub last_chapter: Option<String>,
    pub publication_demographic: Option<Demographic>,
    pub status: Option<MangaStatus>,
    /// Year of release
    pub year: Option<i32>,
    pub content_rating: Option<ContentRating>,
    pub tags: Vec<Tag>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct MangaRequest {
    pub title: LocalizedString,

    #[builder(default)]
    pub alt_titles: Option<Vec<LocalizedString>>,

    #[builder(default)]
    pub description: Option<LocalizedString>,

    #[builder(default)]
    pub authors: Option<Vec<Uuid>>,

    #[builder(default)]
    pub artists: Option<Vec<Uuid>>,

    #[builder(default)]
    pub links: Option<Links>,

    #[builder(default)]
    pub original_language: Option<String>,

    #[builder(default)]
    pub last_volume: Option<String>,

    #[builder(default)]
    pub last_chapter: Option<String>,

    #[builder(default)]
    pub publication_demographic: Option<Demographic>,

    #[builder(default)]
    pub status: Option<MangaStatus>,

    /// Year of release
    #[builder(default)]
    pub year: Option<i32>,

    #[builder(default)]
    pub content_rating: Option<ContentRating>,

    #[builder(default)]
    pub mod_notes: Option<String>,

    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChapterAttributes {
    pub title: String,
    pub volume: Option<String>,
    pub translated_language: String,
    pub hash: String,
    pub data: Vec<String>,
    pub data_saver: Vec<String>,
    pub uploader: Uuid,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub publish_at: DateTime<Utc>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaReadingStatuses {
    pub statuses: HashMap<Uuid, MangaReadingStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MangaReadingStatus {
    Reading,
    OnHold,
    PlanToRead,
    Dropped,
    ReReading,
    Completed,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaReadingStatusBody {
    pub status: MangaReadingStatus,
}

pub type Tag = ApiObject<TagAttributes>;
pub type TagResponse = Result<ApiData<Tag>>;
pub type TagList = Vec<TagResponse>;

pub type Manga = ApiObject<MangaAttributes>;
pub type MangaResponse = Result<ApiData<Manga>>;
pub type MangaList = Results<MangaResponse>;

pub type Chapter = ApiObject<ChapterAttributes>;
pub type ChapterResponse = Result<ApiData<Chapter>>;
pub type ChapterList = Results<ChapterResponse>;