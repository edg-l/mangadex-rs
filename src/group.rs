use crate::{
    common::{PaginationQuery, Results},
    errors::Result,
    user::User,
    ApiData, ApiObject, Client, NoData,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ScanlationGroupAttributes {
    pub name: String,
    pub leader: User,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScanlationGroupType {
    ScanlationGroup,
}

pub type ScanlationGroup = ApiObject<ScanlationGroupAttributes, ScanlationGroupType>;
pub type ScanlationGroupData = ApiData<ScanlationGroup>;
pub type ScanlationGroupResponse = Result<ScanlationGroupData>;
pub type ScanlationGroupList = Results<ScanlationGroupResponse>;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GroupListRequest {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    /// Maximum 100 per request.
    pub ids: Option<Vec<Uuid>>,
    pub name: Option<String>,
}

impl GroupListRequest {
    pub fn new(limit: i32, offset: i32, ids: Vec<Uuid>, name: &str) -> Self {
        Self {
            limit: Some(limit),
            offset: Some(offset),
            ids: Some(ids),
            name: Some(name.to_string()),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateGroupRequest {
    pub name: String,
    pub leader: Option<Uuid>,
    pub members: Option<Vec<Uuid>>,
    pub version: i32,
}

impl CreateGroupRequest {
    pub fn new(name: &str, leader: Option<Uuid>, members: Option<Vec<Uuid>>) -> Self {
        Self {
            name: name.to_string(),
            leader,
            members,
            version: 1,
        }
    }
}

impl Client {
    /// Search for scanlation groups.
    pub async fn list_group(&self, request: &GroupListRequest) -> Result<ScanlationGroupList> {
        let endpoint = self.base_url.join("/group")?;
        let res = self.http.get(endpoint).query(request).send().await?;

        Self::json_api_results(res).await
    }

    /// Create a group.
    ///
    /// Requires auth.
    pub async fn create_group(&self, request: &CreateGroupRequest) -> Result<ScanlationGroupData> {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/group")?;

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .json(&request)
            .send()
            .await?;

        Self::json_api_result(res).await
    }

    /// View a group.
    pub async fn view_group(&self, id: &Uuid) -> Result<ScanlationGroupData> {
        let endpoint = self.base_url.join(&format!("/group/{:x}", id))?;

        let res = self.http.get(endpoint).send().await?;

        Self::json_api_result(res).await
    }

    /// Update a group.
    ///
    /// Requires auth.
    pub async fn update_group(&self, request: &CreateGroupRequest) -> Result<ScanlationGroupData> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/group")?;

        let res = self
            .http
            .put(endpoint)
            .bearer_auth(&tokens.session)
            .json(&request)
            .send()
            .await?;

        Self::json_api_result(res).await
    }

    /// Delete a group.
    ///
    /// Requires auth.
    pub async fn delete_group(&self, id: &Uuid) -> Result<()> {
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

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }

    /// Follow a group.
    ///
    /// Requires auth.
    pub async fn follow_group(&self, id: &Uuid) -> Result<()> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join(&format!("/group/{:x}/follow", id))?;

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }

    /// Unfollow a group.
    ///
    /// Requires auth.
    pub async fn unfollow_group(&self, id: &Uuid) -> Result<()> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join(&format!("/group/{:x}/follow", id))?;

        let res = self
            .http
            .delete(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }

    /// List the followed groups by the logged user.
    ///
    /// Requires auth.
    pub async fn followed_groups(&self, query: &PaginationQuery) -> Result<ScanlationGroupList> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/user/follows/group")?;

        let res = self
            .http
            .get(endpoint)
            .query(&query)
            .bearer_auth(&tokens.session)
            .send()
            .await?;

        Self::json_api_results(res).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn list_group() {
        let client = Client::default();
        let group_request = GroupListRequest::default();
        let groups = client.list_group(&group_request).await.unwrap();
        assert_eq!(groups.offset, 0);
        assert_eq!(groups.limit, 10);
    }
}
