//! Scanlation groups

use crate::{
    model::{NoData, PaginationQuery},
    Result,
};
use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::model::group::*;

/// Lists canlation groups
///
/// Call to `GET /group`
#[derive(Debug, Serialize, Builder, Default)]
#[builder(setter(strip_option), default)]
pub struct ListGroups<'a> {
    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationQuery,

    /// Scanlation group ids (limited to 100 per request)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_id"))]
    pub ids: Vec<&'a Uuid>,

    /// Author name
    pub name: Option<&'a str>,
}

impl_endpoint! {
    GET "/group",
    #[query] ListGroups<'_>,
    ScanlationGroupList
}

/// Create scanlation group (requires authentication)
///
/// Call to `POST /group`
#[derive(Debug, Serialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct CreateGroup<'a> {
    /// Group name
    pub name: &'a str,

    /// Group leader
    pub leader: &'a Uuid,

    /// Memebers list
    #[builder(setter(each = "add_member"))]
    pub members: Vec<&'a Uuid>,

    /// Group version (minimum: 1)
    pub version: i32,
}

impl_endpoint! {
    POST "/group",
    #[body auth] CreateGroup<'_>,
    #[flatten_result] Result<ScanlationGroupData>
}

/// View scanlation group
///
/// Call to `GET /group/{id}`
pub struct ViewGroup<'a> {
    /// Group id
    pub id: &'a Uuid,
}

impl_endpoint! {
    GET ("/group/{:x}", id),
    #[no_data] ViewGroup<'_>,
    #[flatten_result] Result<ScanlationGroupData>
}

/// Update scanlation group (requires auth)
///
/// Call to `PUT /group/{id}`
#[derive(Debug, Serialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct UpdateGroup<'a> {
    /// Group id
    pub id: &'a Uuid,

    /// Group name
    pub name: &'a str,

    /// Group leader
    pub leader: Uuid,

    /// Member list
    #[builder(setter(each = "add_member"))]
    pub members: Vec<Uuid>,

    /// Group version (minimum: 1)
    pub version: i32,
}

impl_endpoint! {
    PUT ("/group/{:x}", id),
    #[body auth] UpdateGroup<'_>,
    #[flatten_result] Result<ScanlationGroupData>
}

/// Delete scanlation group (requires authentication)
///
/// Call to `DELETE /group/{id}`
pub struct DeleteGroup<'a> {
    /// Group id
    pub id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/group/{:x}", id),
    #[no_data auth] DeleteGroup<'_>,
    #[discard_result] Result<NoData>
}

/// Follow scanlation group
///
/// Call to `POST /group/{id}/follow`
pub struct FollowGroup<'a> {
    /// Group id
    pub id: &'a Uuid,
}

impl_endpoint! {
    POST ("/group/{:x}/follow", id),
    #[no_data auth] FollowGroup<'_>,
    #[discard_result] Result<NoData>
}

/// Unfollow scanlation group
///
/// Call to `DELETE /group/{id}/follow`
pub struct UnfollowGroup<'a> {
    /// Group id
    id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/group/{:x}/follow", id),
    #[no_data auth] UnfollowGroup<'_>,
    #[discard_result] Result<NoData>
}

/// Get logged user's followed groups (requires authentication)
///
/// Call to `GET /user/follows/group`
#[derive(Debug, Serialize, Clone)]
pub struct FollowedGroups<'a> {
    /// Pagination parameters
    #[serde(flatten)]
    pagination: &'a PaginationQuery,
}

impl_endpoint! {
    GET "/user/follows/group",
    #[query auth] FollowedGroups<'_>,
    ScanlationGroupList
}

#[cfg(test)]
mod tests {
    use crate::Client;

    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn list_group() {
        let client = Client::default();
        let groups = ListGroupsBuilder::default()
            .build()
            .unwrap()
            .send(&client)
            .await
            .unwrap();
        assert_eq!(groups.offset, 0);
        assert_eq!(groups.limit, 10);
    }
}