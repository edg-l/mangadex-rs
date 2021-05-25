use std::collections::HashMap;

use crate::{
    common::{ApiObject, ApiObjectResult, LocalizedString, Results},
    errors::{ApiErrors, Result},
    Client,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

/// The tag mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TagMode {
    // AND Mode
    And,
    // OR Mode
    Or,
}

/// The status of a manga.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MangaStatus {
    Ongoing,
    Completed,
    Hiatus,
    Cancelled,
}

/// The publication demographic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Demographic {
    Shounen,
    Shoujo,
    Josei,
    Seinen,
    None,
}

/// The content rating of the publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentRating {
    Safe,
    Suggestive,
    Erotica,
    Pornographic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Order {
    #[serde(rename = "createdAt")]
    pub created_at: OrderType,
    #[serde(rename = "updatedAt")]
    pub updated_at: OrderType,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MangaQuery<'a> {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub title: Option<&'a str>,
    pub authors: Option<&'a [Uuid]>,
    pub artists: Option<&'a [Uuid]>,
    pub year: Option<i32>,
    pub included_tags: Option<&'a [Uuid]>,
    pub included_tags_mode: Option<TagMode>,
    pub status: Option<&'a [MangaStatus]>,
    pub original_language: Option<&'a [&'a str]>,
    pub publication_demographic: Option<&'a [Demographic]>,
    pub ids: Option<&'a [Uuid]>,
    pub content_rating: Option<ContentRating>,
    pub created_at_since: Option<DateTime<Utc>>,
    pub updated_at_since: Option<DateTime<Utc>>,
    pub order: Option<Order>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagAttributes {
    pub name: LocalizedString,
    // Known issue: empty descriptions return [] instead of {}
    #[serde(skip)]
    pub description: LocalizedString,
    pub group: String,
    pub version: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: Uuid,
    pub r#type: String,
    pub attributes: TagAttributes,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaAttributes {
    pub title: LocalizedString,
    pub alt_titles: Vec<LocalizedString>,
    // Known issue: empty descriptions return [] instead of {}
    #[serde(skip)]
    pub description: LocalizedString,
    pub is_locked: bool,
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

pub type Manga = ApiObject<MangaAttributes>;
pub type MangaResult = ApiObjectResult<Manga>;
pub type MangaResults = Results<MangaResult>;

impl Client {
    pub async fn list_manga(&self, query: &MangaQuery<'_>) -> Result<MangaResults> {
        let endpoint = self.base_url.join("/manga")?;

        let res = self.http.get(endpoint).query(query).send().await?;
        let res = Self::deserialize_response::<MangaResults, ApiErrors>(res).await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn list_manga() {
        let client = Client::new().unwrap();
        let query = MangaQuery::default();
        let mangas = client.list_manga(&query).await.unwrap();
        println!("{:#?}", mangas);
    }
}
