use std::collections::HashMap;

use crate::{
    common::{ApiObject, ApiObjectResult, LocalizedString, Results},
    errors::{ApiErrors, Result},
    Client, SimpleApiResponse,
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
    // TODO: Known issue: empty descriptions return [] instead of {}
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

pub type Manga = ApiObject<MangaAttributes>;
pub type MangaResult = ApiObjectResult<Manga>;
pub type MangaResults = Results<MangaResult>;

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
        let endpoint = self.base_url.join("/manga/")?.join(&format!("{}", id))?;

        let res = self.http.get(endpoint).send().await?;
        let res = Self::deserialize_response::<MangaResult, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Delete a manga.
    ///
    /// Requires auth.
    pub async fn delete_manga(&self, id: &Uuid) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/manga/")?.join(&format!("{}", id))?;

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
            .join("/manga/")?
            .join(&format!("{}/", manga_id))?
            .join("list/")?
            .join(&format!("{}", list_id))?;

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
            .join("/manga/")?
            .join(&format!("{}/", manga_id))?
            .join("list/")?
            .join(&format!("{}", list_id))?;

        let res = self
            .http
            .delete(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<SimpleApiResponse, ApiErrors>(res).await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
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
}
