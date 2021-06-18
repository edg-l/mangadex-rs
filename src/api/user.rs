use serde::Serialize;

use crate::model::{group::ScanlationGroupList, manga::MangaList};

/// Get logged user's followed groups (requires authentication)
///
/// Call to `GET /user/follows/group`
#[derive(Debug, Serialize, Clone)]
pub struct FollowedGroups {
    /// Page size
    pub limit: Option<i32>,

    /// Page offset
    pub offset: Option<i32>,
}

impl_endpoint! {
    GET "/user/follows/group",
    #[query auth] FollowedGroups,
    ScanlationGroupList
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
