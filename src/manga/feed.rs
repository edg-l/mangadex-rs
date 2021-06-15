use std::borrow::Cow;
use uuid::Uuid;

use super::{ChapterList, MangaFeedQuery};
use crate::Endpoint;

/// Get logged user followed manga feed (requires authentication)
///
/// Returns the feed (chapters list) for the logged user
///
/// Call to `GET /user/follows/manga/feed`
pub struct FollowedMangaFeed<'a> {
    pub query: &'a MangaFeedQuery,
}

impl Endpoint for FollowedMangaFeed<'_> {
    type Query = ();
    type Body = MangaFeedQuery;
    type Response = ChapterList;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("/user/follows/manga/feed")
    }

    fn require_auth(&self) -> bool {
        true
    }

    fn body(&self) -> Option<&Self::Body> {
        Some(&self.query)
    }
}

/// Manga feed
///
/// Returns the feed (chapters list) for the specified manga
///
/// Call to `GET /manga/{id}/feed`
pub struct MangaFeed<'a> {
    pub manga_id: &'a Uuid,
    pub query: &'a MangaFeedQuery,
}

impl Endpoint for MangaFeed<'_> {
    type Query = MangaFeedQuery;
    type Body = ();
    type Response = ChapterList;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/manga/{:x}/feed", self.manga_id))
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.query)
    }
}
