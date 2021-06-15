use reqwest::Method;
use std::borrow::Cow;
use uuid::Uuid;

use crate::{
    manga::{ChapterList, MangaFeedQuery},
    Client, Endpoint, NoData, Result,
};

/// Add manga to custom list (requires authentication)
///
/// Call to `POST /manga/{id}/list/{id}`
pub struct AddMangaToCustomList<'a> {
    pub manga_id: &'a Uuid,
    pub list_id: &'a Uuid,
}

impl Endpoint for AddMangaToCustomList<'_> {
    type Query = ();
    type Body = ();
    type Response = Result<NoData>;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!(
            "/manga/{:x}/list/{:x}",
            self.manga_id, self.list_id
        ))
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn require_auth(&self) -> bool {
        true
    }
}

/// Remove manga from custom list (requires authentication)
///
/// Call to `DELETE /manga/{id}/list/{id}`
pub struct RemoveMangaFromCustomList<'a> {
    pub manga_id: &'a Uuid,
    pub list_id: &'a Uuid,
}

impl Endpoint for RemoveMangaFromCustomList<'_> {
    type Query = ();
    type Body = ();
    type Response = Result<NoData>;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!(
            "/manga/{:x}/list/{:x}",
            self.manga_id, self.list_id
        ))
    }

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn require_auth(&self) -> bool {
        true
    }
}

/// Custom list manga feed
///
/// Call to `GET /list/{id}/feed`
pub struct CustomListMangaFeed<'a> {
    pub list_id: &'a Uuid,
    pub query: &'a MangaFeedQuery,
}

impl Endpoint for CustomListMangaFeed<'_> {
    type Query = MangaFeedQuery;
    type Body = ();
    type Response = ChapterList;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/list/{:x}/feed", self.list_id))
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.query)
    }
}

impl Client {
    /// Add manga to CustomList
    ///
    /// Requires auth.
    pub async fn add_manga_to_custom_list(&self, manga_id: &Uuid, list_id: &Uuid) -> Result<()> {
        self.send_request(&AddMangaToCustomList { manga_id, list_id })
            .await
            .map(|_| ())
    }

    /// Remove manga from CustomList
    ///
    /// Requires auth.
    pub async fn remove_manga_from_custom_list(
        &self,
        manga_id: &Uuid,
        list_id: &Uuid,
    ) -> Result<()> {
        self.send_request(&RemoveMangaFromCustomList { manga_id, list_id })
            .await
            .map(|_| ())
    }

    /// CustomList manga feed.
    pub async fn custom_list_manga_feed(
        &self,
        list_id: &Uuid,
        query: &MangaFeedQuery,
    ) -> Result<ChapterList> {
        self.send_request(&CustomListMangaFeed { list_id, query })
            .await
    }
}
