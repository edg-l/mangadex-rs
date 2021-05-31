use crate::{
    common::{deserialize_null_default, ApiObject, LocalizedString, Results},
    errors::Result,
    ApiData, Client, NoData, OrderType, PaginationQuery, UrlSerdeQS,
};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorOrder {
    pub name: OrderType,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct AuthorQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_id"))]
    pub ids: Vec<Uuid>,

    pub name: Option<String>,
    pub order: Option<AuthorOrder>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorAttributes {
    pub name: String,
    pub image_url: Option<String>,
    pub biography: HashMap<String, String>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type Author = ApiObject<AuthorAttributes>;
pub type AuthorResponse = Result<ApiData<Author>>;
pub type AuthorList = Results<AuthorResponse>;

impl Client {
    /// List authors.
    pub async fn list_authors(&self, query: &AuthorQuery) -> Result<AuthorList> {
        let endpoint = self.base_url.join("/author")?.query_qs(query);
        let res = self.http.get(endpoint).send().await?;

        Self::json_api_results(res).await
    }

    /// Create an author.
    ///
    /// Requires auth.
    pub async fn create_author(&self, name: &str, version: i32) -> AuthorResponse {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/author")?;
        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .json(&serde_json::json!({
                "name": name,
                "version": version
            }))
            .send()
            .await?;

        Self::json_api_result(res).await
    }

    /// Update an author.
    ///
    /// Requires auth.
    pub async fn update_author(&self, id: &Uuid, name: &str, version: i32) -> AuthorResponse {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join(&format!("/author/{:x}", id))?;
        let res = self
            .http
            .put(endpoint)
            .bearer_auth(&tokens.session)
            .json(&serde_json::json!({
                "name": name,
                "version": version
            }))
            .send()
            .await?;

        Self::json_api_result(res).await
    }

    /// View a single author.
    pub async fn view_author(&self, id: &Uuid) -> AuthorResponse {
        let endpoint = self.base_url.join(&format!("/author/{:x}", id))?;
        let res = self.http.get(endpoint).send().await?;

        Self::json_api_result(res).await
    }

    /// Delete an author.
    ///
    /// Requires auth.
    pub async fn delete_author(&self, id: &Uuid) -> Result<()> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join(&format!("/author/{:x}", id))?;

        let res = self
            .http
            .delete(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }
}
