use std::collections::HashMap;

use crate::{
    common::{ApiObject, ApiObjectResult, LocalizedString, Results},
    errors::{ApiErrors, Result},
    ApiResult, Client, PaginationQuery, SimpleApiResponse,
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
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub created_at: OrderType,
    pub updated_at: OrderType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedOrder {
    pub volume: OrderType,
    pub chapter: OrderType,
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MangaFeedQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub translated_language: Option<Vec<String>>,
    pub created_at_since: Option<DateTime<Utc>>,
    pub updated_at_since: Option<DateTime<Utc>>,
    pub publish_at_since: Option<DateTime<Utc>>,
    pub order: Option<FeedOrder>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagAttributes {
    pub name: LocalizedString,
    // TODO: Known issue: empty descriptions return [] instead of {}
    #[serde(skip)]
    pub description: LocalizedString,
    pub group: String,
    pub version: i32,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
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
    // TODO: Known issue: empty descriptions return [] instead of {}
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MangaPayload {
    pub title: LocalizedString,
    pub alt_titles: Option<Vec<LocalizedString>>,
    pub description: Option<LocalizedString>,
    pub authors: Option<Vec<Uuid>>,
    pub artists: Option<Vec<Uuid>>,
    pub links: Option<Links>,
    pub original_language: Option<String>,
    pub last_volume: Option<String>,
    pub last_chapter: Option<String>,
    pub publication_demographic: Option<Demographic>,
    pub status: Option<MangaStatus>,
    /// Year of release
    pub year: Option<i32>,
    pub content_rating: Option<ContentRating>,
    pub mod_notes: Option<String>,
    pub version: i32,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaReadingStatuses {
    pub result: ApiResult,
    pub statuses: HashMap<Uuid, MangaStatus>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaReadingStatus {
    pub result: ApiResult,
    pub status: MangaStatus,
}

pub type Tag = ApiObject<TagAttributes>;
pub type TagResult = ApiObjectResult<Tag>;

pub type Manga = ApiObject<MangaAttributes>;
pub type MangaResult = ApiObjectResult<Manga>;
pub type MangaResults = Results<MangaResult>;

pub type Chapter = ApiObject<ChapterAttributes>;
pub type ChapterResult = ApiObjectResult<Chapter>;
pub type ChapterResults = Results<ChapterResult>;

impl Client {
    /// List mangas.
    pub async fn list_manga(&self, query: &MangaQuery<'_>) -> Result<MangaResults> {
        let endpoint = self.base_url.join("/manga")?;

        let res = self.http.get(endpoint).query(query).send().await?;
        let res = Self::deserialize_response::<MangaResults, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Create a manga.
    ///
    /// Requires auth.
    pub async fn create_manga(&self, request: &MangaPayload) -> Result<MangaResult> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/manga")?;

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .json(request)
            .send()
            .await?;
        let res = Self::deserialize_response::<MangaResult, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Update a manga.
    ///
    /// Requires auth.
    pub async fn update_manga(&self, request: &MangaPayload) -> Result<MangaResult> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/manga")?;

        let res = self
            .http
            .put(endpoint)
            .bearer_auth(&tokens.session)
            .json(request)
            .send()
            .await?;
        let res = Self::deserialize_response::<MangaResult, ApiErrors>(res).await?;

        Ok(res)
    }

    /// View a single manga.
    pub async fn view_manga(&self, id: &Uuid) -> Result<MangaResult> {
        let endpoint = self.base_url.join(&format!("/manga/{:x}", id))?;

        let res = self.http.get(endpoint).send().await?;
        let res = Self::deserialize_response::<MangaResult, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Delete a manga.
    ///
    /// Requires auth.
    pub async fn delete_manga(&self, id: &Uuid) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join(&format!("/manga/{:x}", id))?;

        let res = self
            .http
            .delete(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<SimpleApiResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Add manga to CustomList
    ///
    /// Requires auth.
    pub async fn add_manga_to_custom_list(
        &self,
        manga_id: &Uuid,
        list_id: &Uuid,
    ) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self
            .base_url
            .join(&format!("/manga/{:x}/list/{:x}", manga_id, list_id))?;

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<SimpleApiResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Remove manga from CustomList
    ///
    /// Requires auth.
    pub async fn remove_manga_from_custom_list(
        &self,
        manga_id: &Uuid,
        list_id: &Uuid,
    ) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self
            .base_url
            .join(&format!("/manga/{:x}/list/{:x}", manga_id, list_id))?;

        let res = self
            .http
            .delete(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<SimpleApiResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Get logged User followed Manga feed
    pub async fn followed_manga_feed(&self, query: &MangaFeedQuery) -> Result<ChapterResults> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/user/follows/manga/feed")?;

        let res = self
            .http
            .get(endpoint)
            .bearer_auth(&tokens.session)
            .json(query)
            .send()
            .await?;
        let res = Self::deserialize_response::<ChapterResults, ApiErrors>(res).await?;

        Ok(res)
    }

    pub async fn unfollow_manga(&self, manga_id: &Uuid) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self
            .base_url
            .join(&format!("/manga/{:x}/follow", manga_id))?;

        let res = self
            .http
            .delete(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<SimpleApiResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    pub async fn follow_manga(&self, manga_id: &Uuid) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self
            .base_url
            .join(&format!("/manga/{:x}/follow", manga_id))?;

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<SimpleApiResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    pub async fn manga_feed(
        &self,
        manga_id: &Uuid,
        query: &MangaFeedQuery,
    ) -> Result<ChapterResults> {
        let endpoint = self.base_url.join(&format!("/manga/{:x}/feed", manga_id))?;

        let res = self.http.get(endpoint).json(query).send().await?;
        let res = Self::deserialize_response::<ChapterResults, ApiErrors>(res).await?;

        Ok(res)
    }

    /// A list of chapter ids that are marked as read for the specified manga
    pub async fn manga_read_markers(&self, manga_id: &Uuid) -> Result<ApiObjectResult<Vec<Uuid>>> {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join(&format!("/manga/{:x}/read", manga_id))?;

        let res = self
            .http
            .get(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<_, ApiErrors>(res).await?;

        Ok(res)
    }

    /// A list of chapter ids that are marked as read for the given manga ids
    pub async fn manga_read_markers_more(
        &self,
        manga_ids: &[Uuid],
    ) -> Result<ApiObjectResult<Vec<Uuid>>> {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/manga/read")?;

        let res = self
            .http
            .get(endpoint)
            .bearer_auth(&tokens.session)
            .query(
                &manga_ids
                    .iter()
                    .map(|id| ("ids", id))
                    .collect::<Vec<(&str, &Uuid)>>(),
            )
            .send()
            .await?;
        let res = Self::deserialize_response::<_, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Get a random Manga
    pub async fn random_manga(&self) -> Result<MangaResult> {
        let endpoint = self.base_url.join("/manga/random")?;

        let res = self.http.get(endpoint).send().await?;
        let res = Self::deserialize_response::<MangaResult, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Get a random Manga
    pub async fn tag_list(&self) -> Result<Vec<TagResult>> {
        let endpoint = self.base_url.join("/manga/tag")?;

        let res = self.http.get(endpoint).send().await?;
        let res = Self::deserialize_response::<_, ApiErrors>(res).await?;

        Ok(res)
    }

    pub async fn followed_manga_list(&self, query: &PaginationQuery) -> Result<MangaResults> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/user/follows/manga")?;

        let res = self
            .http
            .get(endpoint)
            .bearer_auth(&tokens.session)
            .json(query)
            .send()
            .await?;
        let res = Self::deserialize_response::<_, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Get all Manga reading status for logged User
    pub async fn all_manga_reading_status(
        &self,
        status: Option<MangaStatus>,
    ) -> Result<MangaReadingStatuses> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/manga/status")?;

        let mut req = self.http.get(endpoint).bearer_auth(&tokens.session);

        if let Some(status) = status {
            req = req.query(&[("status", status)]);
        }

        let res = req.send().await?;
        let res = Self::deserialize_response::<_, ApiErrors>(res).await?;

        Ok(res)
    }

    pub async fn manga_reading_status(&self, manga_id: &Uuid) -> Result<MangaReadingStatus> {
        let tokens = self.require_tokens()?;

        let endpoint = self
            .base_url
            .join(&format!("/manga/{:x}/status", manga_id))?;

        let res = self
            .http
            .get(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<_, ApiErrors>(res).await?;

        Ok(res)
    }

    pub async fn update_manga_reading_status(
        &self,
        manga_id: &Uuid,
        status: MangaStatus,
    ) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self
            .base_url
            .join(&format!("/manga/{:x}/status", manga_id))?;

        let mut payload = HashMap::with_capacity(1);
        payload.insert("status", status);

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .json(&payload)
            .send()
            .await?;
        let res = Self::deserialize_response::<_, ApiErrors>(res).await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::ResourceType;

    use super::*;
    use chrono::prelude::*;
    use isolanguage_1::LanguageCode;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn list_manga() {
        let client = Client::new().unwrap();
        let query = MangaQuery::default();
        let manga = client.list_manga(&query).await.unwrap();
        assert_eq!(manga.offset, 0);
        assert_eq!(manga.limit, 10);
    }

    #[tokio::test]
    async fn view_manga() {
        let id = Uuid::parse_str("32d76d19-8a05-4db0-9fc2-e0b0648fe9d0").unwrap();
        let client = Client::new().unwrap();
        let manga_result = client.view_manga(&id).await.unwrap();

        let manga = manga_result.data;
        assert_eq!(manga.id, id);
        assert_eq!(
            manga
                .attributes
                .title
                .get(&LanguageCode::En)
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
        let client = Client::new().unwrap();
        let manga_result = client.random_manga().await.unwrap();
        let manga = manga_result.data;
        assert_eq!(manga.r#type, ResourceType::Manga);
    }

    #[tokio::test]
    async fn tag_list() {
        let client = Client::new().unwrap();
        let tag_results = client.tag_list().await.unwrap();

        for result in &tag_results {
            let tag = &result.data;
            assert_eq!(tag.r#type, ResourceType::Tag);
        }
    }
}
