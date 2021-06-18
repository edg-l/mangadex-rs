//! Custom lists

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    schema::{NoData, PaginationQuery},
    Result,
};

use crate::schema::list::*;

/// Create custom list (requires authentication)
///
/// Call to `POST /list`
#[derive(Debug, Serialize, Clone, Builder)]
pub struct CreateCustomList<'a> {
    /// List name
    pub name: &'a str,

    /// List visibility
    #[builder(default = "CustomListVisibility::Private")]
    pub visibility: CustomListVisibility,

    /// List of manga ids
    #[builder(setter(each = "add_manga"))]
    pub manga: Vec<&'a Uuid>,

    /// List version
    pub version: i32,
}

impl_endpoint! {
    POST "/list",
    #[body auth] CreateCustomList<'_>,
    #[flatten_result] CustomListResponse
}

/// Get custom list
///
/// Call to `GET /list/{id}`
#[derive(Debug, Clone)]
pub struct GetCustomList<'a> {
    /// Custom list id
    id: &'a Uuid,
}

impl_endpoint! {
    GET ("/list/{:x}", id),
    #[no_data] GetCustomList<'_>,
    #[flatten_result] CustomListResponse
}

/// Update custom list (requires authentication)
///
/// Call to `PUT /list/{id}`
#[derive(Debug, Serialize, Clone, Builder)]
pub struct UpdateCustomList<'a> {
    /// List id
    #[serde(skip)]
    pub id: &'a Uuid,

    /// List name
    pub name: &'a str,

    /// List visibility
    pub visibility: CustomListVisibility,

    /// List of manga ids
    #[builder(setter(each = "add_manga"))]
    pub manga: Vec<&'a Uuid>,

    /// List version
    pub version: i32,
}

impl_endpoint! {
    PUT ("/list/{:x}", id),
    #[body auth] UpdateCustomList<'_>,
    #[flatten_result] CustomListResponse
}

/// Delete custom list (requires authentication)
///
/// Call to `DELETE /list/{id}`
#[derive(Debug, Clone)]
pub struct DeleteCustomList<'a> {
    /// List id
    pub id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/list/{:x}", id),
    #[no_data auth] DeleteCustomList<'_>,
    #[discard_result] Result<NoData>
}

/// Add manga to custom list (requires authentication)
///
/// Call to `POST /manga/{id}/list/{id}`
#[derive(Debug, Clone)]
pub struct AddMangaToCustomList<'a> {
    /// Manga to add
    pub manga_id: &'a Uuid,

    /// List to edit
    pub list_id: &'a Uuid,
}

impl_endpoint! {
    POST ("/manga/{:x}/list/{:x}", manga_id, list_id),
    #[no_data auth] AddMangaToCustomList<'_>,
    #[discard_result] Result<NoData>
}

/// Remove manga from custom list (requires authentication)
///
/// Call to `DELETE /manga/{id}/list/{id}`
#[derive(Debug, Clone)]
pub struct RemoveMangaFromCustomList<'a> {
    /// Manga to remove
    pub manga_id: &'a Uuid,

    /// List to edit
    pub list_id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/manga/{:x}/list/{:x}", manga_id, list_id),
    #[no_data auth] RemoveMangaFromCustomList<'_>,
    #[discard_result] Result<NoData>
}

/// Get all of logged user's custom lists (requires authentication)
///
/// This will list public and private custom lists
///
/// Call to `GET /user/list`
#[derive(Debug, Serialize, Clone)]
pub struct GetLoggedUserCustomLists {
    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationQuery,
}

impl_endpoint! {
    GET "/user/list",
    #[query auth] GetLoggedUserCustomLists,
    CustomListList
}

/// Get public custom lists for specific user (requires authentication)
///
/// Call to `GET /user/{user_id}/list`
#[derive(Debug, Serialize, Clone)]
pub struct GetUserCustomLists<'a> {
    /// User id
    #[serde(skip)]
    pub user_id: &'a Uuid,

    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationQuery,
}

impl_endpoint! {
    GET ("/user/{:x}/list", user_id),
    #[query auth] GetUserCustomLists<'_>,
    CustomListList
}
