use serde::Serialize;
use uuid::Uuid;

use crate::{
    manga::{ChapterList, MangaFeedQuery},
    NoData, Result,
};

/// Add manga to custom list (requires authentication)
///
/// Call to `POST /manga/{id}/list/{id}`
pub struct AddMangaToCustomList<'a> {
    pub manga_id: &'a Uuid,
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
pub struct RemoveMangaFromCustomList<'a> {
    pub manga_id: &'a Uuid,
    pub list_id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/manga/{:x}/list/{:x}", manga_id, list_id),
    #[no_data auth] RemoveMangaFromCustomList<'_>,
    #[discard_result] Result<NoData>
}

/// Custom list manga feed
///
/// Call to `GET /list/{id}/feed`
#[derive(Debug, Serialize, Clone)]
pub struct CustomListMangaFeed<'a> {
    #[serde(skip)]
    pub list_id: &'a Uuid,
    #[serde(flatten)]
    pub query: &'a MangaFeedQuery,
}

impl_endpoint! {
    GET ("/list/{:x}/feed", list_id),
    #[query] CustomListMangaFeed<'_>,
    ChapterList
}
