use crate::{
    common::{PaginationQuery, Results, UrlSerdeQS},
    errors::Result,
    user::User,
    ApiData, ApiObject, Client, NoData,
};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
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

#[derive(Debug, Builder, Serialize, Deserialize, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ListGroupReq {
    pub limit: Option<i32>,

    pub offset: Option<i32>,

    /// Maximum 100 per request.
    pub ids: Option<Vec<Uuid>>,

    pub name: Option<String>,
}

impl_endpoint! {
    GET "/group",
    #[query] ListGroupReq,
    ScanlationGroupList
}

impl ListGroupReq {
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
#[derive(Debug, Builder, Serialize, Deserialize, Clone)]
#[builder(setter(into, strip_option))]
pub struct CreateGroupReq {
    pub name: String,

    #[builder(default)]
    pub leader: Option<Uuid>,

    #[builder(default)]
    pub members: Option<Vec<Uuid>>,

    pub version: i32,
}

impl_endpoint! {
    POST "/group",
    #[body auth] CreateGroupReq,
    #[flatten_result] Result<ScanlationGroupData>
}

pub struct ViewGroupReq<'a> {
    id: &'a Uuid,
}

impl_endpoint! {
    GET ("/group/{:x}", id),
    #[no_data] ViewGroupReq<'_>,
    #[flatten_result] Result<ScanlationGroupData>
}

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize, Deserialize, Clone)]
#[builder(setter(into, strip_option))]
pub struct UpdateGroupReq {
    pub name: String,

    #[builder(default)]
    pub leader: Option<Uuid>,

    #[builder(default)]
    pub members: Option<Vec<Uuid>>,

    pub version: i32,
}

impl_endpoint! {
    PUT "/group",
    #[body auth] UpdateGroupReq,
    #[flatten_result] Result<ScanlationGroupData>
}

pub struct DeleteGroupReq<'a> {
    id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/group/{:x}", id),
    #[no_data auth] DeleteGroupReq<'_>,
    #[discard_result] Result<NoData>
}

pub struct FollowGroupReq<'a> {
    id: &'a Uuid,
}

impl_endpoint! {
    POST ("/group/{:x}/follow", id),
    #[no_data auth] FollowGroupReq<'_>,
    #[discard_result] Result<NoData>
}

pub struct UnfollowGroupReq<'a> {
    id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/group/{:x}/follow", id),
    #[no_data auth] UnfollowGroupReq<'_>,
    #[discard_result] Result<NoData>
}

#[derive(Debug, Serialize, Clone)]
pub struct FollowedGroupsReq<'a> {
    #[serde(flatten)]
    pagination: &'a PaginationQuery,
}

impl_endpoint! {
    GET "/user/follows/group",
    #[query auth] FollowedGroupsReq<'_>,
    ScanlationGroupList
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn list_group() {
        let client = Client::default();
        let groups = ListGroupReqBuilder::default()
            .build()
            .unwrap()
            .send(&client)
            .await
            .unwrap();
        assert_eq!(groups.offset, 0);
        assert_eq!(groups.limit, 10);
    }
}
