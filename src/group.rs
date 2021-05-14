use crate::{
    common::{ApiResult, ListRequest, Relationship, SimpleApiResponse},
    errors::{ApiErrors, Result},
    user::User,
    Client,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ScanlationGroupAttributes {
    pub name: String,
    pub leader: User,
    pub version: i32,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: DateTime<Utc>,
    #[serde(rename(deserialize = "updatedAt"))]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ScanlationGroup {
    pub id: Uuid,
    pub r#type: String,
    pub attributes: ScanlationGroupAttributes,
}

#[derive(Debug, Deserialize)]
pub struct ScanlationGroupResponse {
    pub result: ApiResult,
    pub data: ScanlationGroup,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Deserialize)]
pub struct GroupListResponse {
    pub results: Vec<ScanlationGroupResponse>,
    pub limit: i32,
    pub offset: i32,
    pub total: i32,
}

#[derive(Debug, Serialize)]
pub struct GroupListRequest<'a> {
    pub limit: i32,
    pub offset: i32,
    /// Maximum 100 per request.
    pub ids: &'a [Uuid],
    pub name: &'a str,
}

impl<'a> GroupListRequest<'a> {
    pub fn new(limit: i32, offset: i32, ids: &'a [Uuid], name: &'a str) -> Self {
        Self {
            limit,
            offset,
            ids,
            name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateGroupRequest<'a> {
    pub name: &'a str,
    pub leader: Option<Uuid>,
    pub members: Option<&'a [Uuid]>,
    pub version: i32,
}

impl<'a> CreateGroupRequest<'a> {
    pub fn new(name: &'a str, leader: Option<Uuid>, members: Option<&'a [Uuid]>) -> Self {
        Self {
            name,
            leader,
            members,
            version: 1,
        }
    }
}

impl Client {
    /// Search for scanlation groups.
    pub async fn list_group(&self, request: &GroupListRequest<'_>) -> Result<GroupListResponse> {
        let endpoint = self.base_url.join("/group")?;

        let res = self.http.get(endpoint).query(&request).send().await?;
        let res = Self::deserialize_response::<GroupListResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Create a group.
    pub async fn create_group(
        &self,
        request: &CreateGroupRequest<'_>,
    ) -> Result<ScanlationGroupResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/group")?;

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .json(&request)
            .send()
            .await?;
        let res = Self::deserialize_response::<ScanlationGroupResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    /// View a group.
    pub async fn view_group(&self, id: &Uuid) -> Result<ScanlationGroupResponse> {
        let mut buffer = Uuid::encode_buffer();
        let id_str = id.to_hyphenated_ref().encode_lower(&mut buffer);
        let endpoint = self.base_url.join("/group/")?.join(id_str)?;

        let res = self.http.get(endpoint).send().await?;
        let res = Self::deserialize_response::<ScanlationGroupResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Update a group.
    pub async fn update_group(
        &self,
        request: &CreateGroupRequest<'_>,
    ) -> Result<ScanlationGroupResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/group")?;

        let res = self
            .http
            .put(endpoint)
            .bearer_auth(&tokens.session)
            .json(&request)
            .send()
            .await?;
        let res = Self::deserialize_response::<ScanlationGroupResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Delete a group.
    pub async fn delete_group(&self, id: &Uuid) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let mut buffer = Uuid::encode_buffer();
        let id_str = id.to_hyphenated_ref().encode_lower(&mut buffer);
        let endpoint = self.base_url.join("/group/")?.join(id_str)?;

        let res = self
            .http
            .delete(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<SimpleApiResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Follow a group.
    pub async fn follow_group(&self, id: &Uuid) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let mut buffer = Uuid::encode_buffer();
        let id_str = id.to_hyphenated_ref().encode_lower(&mut buffer);

        let endpoint = self
            .base_url
            .join("/group/")?
            .join(&format!("{}/", id_str))?
            .join("follow")?;

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<SimpleApiResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Unfollow a group.
    pub async fn unfollow_group(&self, id: &Uuid) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let mut buffer = Uuid::encode_buffer();
        let id_str = id.to_hyphenated_ref().encode_lower(&mut buffer);

        let endpoint = self
            .base_url
            .join("/group/")?
            .join(&format!("{}/", id_str))?
            .join("follow")?;

        let res = self
            .http
            .delete(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<SimpleApiResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    /// List the followed groups by the logged user.
    pub async fn followed_groups(&self, request: &ListRequest) -> Result<GroupListResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/user/follows/group")?;

        let res = self
            .http
            .delete(endpoint)
            .query(&request)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<GroupListResponse, ApiErrors>(res).await?;

        Ok(res)
    }
}
