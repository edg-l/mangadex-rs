use crate::{
    common::{ApiObject, ApiObjectResult, PaginationQuery, Results, SimpleApiResponse},
    errors::{ApiErrors, Result},
    user::User,
    Client,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
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

pub type ScanlationGroup = ApiObject<ScanlationGroupAttributes>;
pub type ScanlationGroupResult = ApiObjectResult<ScanlationGroup>;
pub type GroupResults = Results<ScanlationGroupResult>;

#[derive(Debug, Serialize, Default)]
pub struct GroupListRequest<'a> {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    /// Maximum 100 per request.
    pub ids: Option<&'a [Uuid]>,
    pub name: Option<&'a str>,
}

impl<'a> GroupListRequest<'a> {
    pub fn new(limit: i32, offset: i32, ids: &'a [Uuid], name: &'a str) -> Self {
        Self {
            limit: Some(limit),
            offset: Some(offset),
            ids: Some(ids),
            name: Some(name),
        }
    }
}

#[skip_serializing_none]
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
    pub async fn list_group(&self, request: &GroupListRequest<'_>) -> Result<GroupResults> {
        let endpoint = self.base_url.join("/group")?;

        let res = self.http.get(endpoint).query(request).send().await?;
        let res = Self::deserialize_response::<GroupResults, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Create a group.
    ///
    /// Requires auth.
    pub async fn create_group(
        &self,
        request: &CreateGroupRequest<'_>,
    ) -> Result<ScanlationGroupResult> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/group")?;

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .json(&request)
            .send()
            .await?;
        let res = Self::deserialize_response::<ScanlationGroupResult, ApiErrors>(res).await?;

        Ok(res)
    }

    /// View a group.
    pub async fn view_group(&self, id: &Uuid) -> Result<ScanlationGroupResult> {
        let endpoint = self.base_url.join(&format!("/group/{:x}", id))?;

        let res = self.http.get(endpoint).send().await?;
        let res = Self::deserialize_response::<ScanlationGroupResult, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Update a group.
    ///
    /// Requires auth.
    pub async fn update_group(
        &self,
        request: &CreateGroupRequest<'_>,
    ) -> Result<ScanlationGroupResult> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/group")?;

        let res = self
            .http
            .put(endpoint)
            .bearer_auth(&tokens.session)
            .json(&request)
            .send()
            .await?;
        let res = Self::deserialize_response::<ScanlationGroupResult, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Delete a group.
    ///
    /// Requires auth.
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
    ///
    /// Requires auth.
    pub async fn follow_group(&self, id: &Uuid) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join(&format!("/group/{:x}/follow", id))?;

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
    ///
    /// Requires auth.
    pub async fn unfollow_group(&self, id: &Uuid) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join(&format!("/group/{:x}/follow", id))?;

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
    ///
    /// Requires auth.
    pub async fn followed_groups(&self, query: &PaginationQuery) -> Result<GroupResults> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/user/follows/group")?;

        let res = self
            .http
            .get(endpoint)
            .query(&query)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<GroupResults, ApiErrors>(res).await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn list_group() {
        let client = Client::new().unwrap();
        let group_request = GroupListRequest::default();
        let groups = client.list_group(&group_request).await.unwrap();
        assert_eq!(groups.offset, 0);
        assert_eq!(groups.limit, 10);
    }
}
