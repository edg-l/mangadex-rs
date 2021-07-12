//! User data

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::{group::ScanlationGroupList, manga::MangaList, user::*, NoData};
use crate::Result;

/// List users (requires authentication)
///
/// Call to `GET /user`
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[builder(setter(strip_option), default)]
pub struct ListUsers<'a> {
    /// Page size
    pub limit: Option<i32>,

    /// Page offset
    pub offset: Option<i32>,

    /// User ids
    #[builder(setter(each = "add_user"))]
    #[serde(rename = "ids")]
    pub user_ids: Vec<&'a Uuid>,

    /// Username
    pub username: Option<&'a str>,

    /// Sort order
    pub order: Option<UserOrder>,
}

impl_endpoint! {
    GET "/user",
    #[query auth] ListUsers<'_>,
    UserList
}

/// Get user
///
/// Call to `GET /user/{user_id}`
#[derive(Debug, Clone)]
pub struct GetUser<'a> {
    /// The user id.
    pub user_id: &'a Uuid,
}

impl_endpoint! {
    GET ("/user/{:x}", user_id),
    #[no_data] GetUser<'_>,
    #[flatten_result] UserResponse
}

/// Delete user
///
/// Call to `DELETE /user/{user_id}`
#[derive(Debug, Clone)]
pub struct DeleteUser<'a> {
    /// The user id.
    pub user_id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/user/{:x}", user_id),
    #[no_data auth] DeleteUser<'_>,
    #[discard_result] Result<NoData>
}

/// Approve user deletion
///
/// Call to `POST /user/delete/{code}`
#[derive(Debug, Clone)]
pub struct ApproveUserDeletion<'a> {
    /// The code.
    pub code: &'a Uuid,
}

impl_endpoint! {
    POST ("/user/delete/{:x}", code),
    #[no_data auth] ApproveUserDeletion<'_>,
    #[discard_result] Result<NoData>
}

/// Update user password
///
/// Call to `POST /user/password`
#[derive(Debug, Serialize, Clone)]
pub struct UpdatePassword<'a> {
    /// The old password.
    pub old_password: &'a str,
    /// The new password.
    pub new_password: &'a str,
}

impl_endpoint! {
    POST "/user/password",
    #[body auth] UpdatePassword<'_>,
    #[discard_result] Result<NoData>
}

/// Update user email
///
/// Call to `POST /user/email`
#[derive(Debug, Serialize, Clone)]
pub struct UpdateEmail<'a> {
    pub email: &'a str,
}

impl_endpoint! {
    POST "/user/email",
    #[body auth] UpdateEmail<'_>,
    #[discard_result] Result<NoData>
}

#[derive(Debug, Clone)]
pub struct GetLoggedUser;

impl_endpoint! {
    GET "/user/me",
    #[no_data auth] GetLoggedUser,
    #[flatten_result] UserResponse
}

/// Get logged user's followed groups (requires authentication)
///
/// Call to `GET /user/follows/group`
#[derive(Debug, Serialize, Clone)]
pub struct ListFollowedGroups {
    /// Page size
    pub limit: Option<i32>,

    /// Page offset
    pub offset: Option<i32>,
}

impl_endpoint! {
    GET "/user/follows/group",
    #[query auth] ListFollowedGroups,
    ScanlationGroupList
}

/// Get logged user's followed users (requires authentication)
///
/// Call to `GET /user/follows/user`
#[derive(Debug, Serialize, Clone)]
pub struct ListFollowedUsers {
    /// Page size
    pub limit: Option<i32>,

    /// Page offset
    pub offset: Option<i32>,
}

impl_endpoint! {
    GET "/user/follows/user",
    #[query auth] ListFollowedUsers,
    UserList
}

/// Get logged user followed manga list (requires authentication)
///
/// Call to `GET /usr/follows/manga`
#[derive(Debug, Serialize, Clone)]
pub struct ListFollowedManga {
    /// Page size
    pub limit: Option<i32>,

    /// Page offset
    pub offset: Option<i32>,
}

impl_endpoint! {
    GET "/user/follows/manga",
    #[query auth] ListFollowedManga,
    MangaList
}
