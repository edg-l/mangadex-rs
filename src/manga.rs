use std::collections::HashMap;

use crate::{
    common::{ApiObject, LocalizedString, Results},
    errors::Result,
    ApiData, Client, NoData, PaginationQuery,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

/// The tag mode.
#[derive(Debug, Serialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TagMode {
    // AND Mode
    And,
    // OR Mode
    Or,
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Asc,
    Desc,
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
#[derive(Debug, Default, Serialize, Clone, Hash, PartialEq, Eq)]
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
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
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
    pub is_locked: bool,
    #[serde(default)]
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
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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
struct MangaReadingStatuses {
    pub statuses: HashMap<Uuid, MangaStatus>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MangaReadingStatus {
    pub status: MangaStatus,
}

pub type Tag = ApiObject<TagAttributes>;
pub type TagResponse = Result<ApiData<Tag>>;

pub type Manga = ApiObject<MangaAttributes>;
pub type MangaResponse = Result<ApiData<Manga>>;
pub type MangaList = Results<MangaResponse>;

pub type Chapter = ApiObject<ChapterAttributes>;
pub type ChapterResponse = Result<ApiData<Chapter>>;
pub type ChapterList = Results<ChapterResponse>;

impl Client {
    /// List mangas.
    pub async fn list_manga(&self, query: &MangaQuery<'_>) -> Result<MangaList> {
        let endpoint = self.base_url.join("/manga")?;
        let res = self.http.get(endpoint).query(query).send().await?;

        Self::json_api_results(res).await
    }

    /// Create a manga.
    ///
    /// Requires auth.
    pub async fn create_manga(&self, request: &MangaPayload) -> MangaResponse {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/manga")?;
        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .json(request)
            .send()
            .await?;

        Self::json_api_result(res).await
    }

    /// Update a manga.
    ///
    /// Requires auth.
    pub async fn update_manga(&self, request: &MangaPayload) -> MangaResponse {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/manga")?;
        let res = self
            .http
            .put(endpoint)
            .bearer_auth(&tokens.session)
            .json(request)
            .send()
            .await?;

        Self::json_api_result(res).await
    }

    /// View a single manga.
    pub async fn view_manga(&self, id: &Uuid) -> MangaResponse {
        let endpoint = self.base_url.join(&format!("/manga/{:x}", id))?;
        let res = self.http.get(endpoint).send().await?;

        Self::json_api_result(res).await
    }

    /// Delete a manga.
    ///
    /// Requires auth.
    pub async fn delete_manga(&self, id: &Uuid) -> Result<()> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join(&format!("/manga/{:x}", id))?;

        let res = self
            .http
            .delete(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }

    /// Add manga to CustomList
    ///
    /// Requires auth.
    pub async fn add_manga_to_custom_list(&self, manga_id: &Uuid, list_id: &Uuid) -> Result<()> {
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

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }

    /// Remove manga from CustomList
    ///
    /// Requires auth.
    pub async fn remove_manga_from_custom_list(
        &self,
        manga_id: &Uuid,
        list_id: &Uuid,
    ) -> Result<()> {
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

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }

    /// Get logged User followed Manga feed
    pub async fn followed_manga_feed(&self, query: &MangaFeedQuery) -> Result<ChapterList> {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/user/follows/manga/feed")?;
        let res = self
            .http
            .get(endpoint)
            .bearer_auth(&tokens.session)
            .json(query)
            .send()
            .await?;

        Self::json_api_results(res).await
    }

    pub async fn unfollow_manga(&self, manga_id: &Uuid) -> Result<()> {
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

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }

    pub async fn follow_manga(&self, manga_id: &Uuid) -> Result<()> {
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

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }

    pub async fn manga_feed(&self, manga_id: &Uuid, query: &MangaFeedQuery) -> Result<ChapterList> {
        let endpoint = self.base_url.join(&format!("/manga/{:x}/feed", manga_id))?;
        let res = self.http.get(endpoint).json(query).send().await?;

        Self::json_api_results(res).await
    }

    /// A list of chapter ids that are marked as read for the specified manga
    pub async fn manga_read_markers(&self, manga_id: &Uuid) -> Result<ApiData<Vec<Uuid>>> {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join(&format!("/manga/{:x}/read", manga_id))?;

        let res = self
            .http
            .get(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;

        Self::json_api_result(res).await
    }

    /// A list of chapter ids that are marked as read for the given manga ids
    pub async fn manga_read_markers_more(&self, manga_ids: &[Uuid]) -> Result<ApiData<Vec<Uuid>>> {
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

        Self::json_api_result(res).await
    }

    /// Get a random Manga
    pub async fn random_manga(&self) -> MangaResponse {
        let endpoint = self.base_url.join("/manga/random")?;
        let res = self.http.get(endpoint).send().await?;

        Self::json_api_result(res).await
    }

    pub async fn tag_list(&self) -> Result<Vec<TagResponse>> {
        let endpoint = self.base_url.join("/manga/tag")?;
        let res = self.http.get(endpoint).send().await?;

        Self::json_api_result_vec(res).await
    }

    pub async fn followed_manga_list(&self, query: &PaginationQuery) -> Result<MangaList> {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/user/follows/manga")?;

        let res = self
            .http
            .get(endpoint)
            .bearer_auth(&tokens.session)
            .json(query)
            .send()
            .await?;

        Self::json_api_results(res).await
    }

    /// Get all Manga reading status for logged User
    pub async fn all_manga_reading_status(
        &self,
        status: Option<MangaStatus>,
    ) -> Result<HashMap<Uuid, MangaStatus>> {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/manga/status")?;

        let mut req = self.http.get(endpoint).bearer_auth(&tokens.session);
        if let Some(status) = status {
            req = req.query(&[("status", status)]);
        }

        let res = req.send().await?;

        Self::json_api_result::<MangaReadingStatuses>(res)
            .await
            .map(|s| s.statuses)
    }

    pub async fn manga_reading_status(&self, manga_id: &Uuid) -> Result<MangaStatus> {
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

        Self::json_api_result::<MangaReadingStatus>(res)
            .await
            .map(|s| s.status)
    }

    pub async fn update_manga_reading_status(
        &self,
        manga_id: &Uuid,
        status: MangaStatus,
    ) -> Result<()> {
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

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
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
        let client = Client::default();
        let query = MangaQuery::default();
        let manga = client.list_manga(&query).await.unwrap();
        assert_eq!(manga.offset, 0);
        assert_eq!(manga.limit, 10);
    }

    #[tokio::test]
    async fn view_manga() {
        let id = Uuid::parse_str("32d76d19-8a05-4db0-9fc2-e0b0648fe9d0").unwrap();
        let client = Client::default();
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
        let client = Client::default();
        let manga_result = client.random_manga().await.unwrap();
        let manga = manga_result.data;
        assert_eq!(manga.r#type, ResourceType::Manga);
    }

    #[tokio::test]
    async fn tag_list() {
        let client = Client::default();
        let tag_results = client.tag_list().await.unwrap();

        for result in &tag_results {
            let tag = &result.as_ref().unwrap().data;
            assert_eq!(tag.r#type, ResourceType::Tag);
        }
    }
}
