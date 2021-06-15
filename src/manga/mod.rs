use crate::{errors::Result, ApiData, Client, Endpoint, NoData, PaginationQuery};

use reqwest::Method;
use std::{borrow::Cow, collections::HashMap};
use uuid::Uuid;

mod tests;

mod types;
pub use types::*;

mod feed;
pub use feed::*;

mod status;
pub use status::*;

/// Manga list
///
/// Search a list of manga
///
/// Call to `GET /manga`
pub struct ListManga<'a> {
    pub query: &'a MangaQuery,
}

impl Endpoint for ListManga<'_> {
    type Query = MangaQuery;
    type Body = ();
    type Response = MangaList;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed("/manga")
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.query)
    }
}

/// Create manga (requires authentication)
///
/// Create a new manga
///
/// Call to `POST /manga`
pub struct CreateManga<'a> {
    pub request: &'a MangaRequest,
}

impl Endpoint for CreateManga<'_> {
    type Query = ();
    type Body = MangaRequest;
    type Response = MangaResponse;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("/manga")
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn require_auth(&self) -> bool {
        true
    }

    fn body(&self) -> Option<&Self::Body> {
        Some(&self.request)
    }
}

/// Update manga (requires authentication)
///
/// Update an existing manga
///
/// Call to `PUT /manga/{id}`
pub struct UpdateManga<'a> {
    pub id: &'a Uuid,
    pub request: &'a MangaRequest,
}

impl Endpoint for UpdateManga<'_> {
    type Query = ();
    type Body = MangaRequest;
    type Response = MangaResponse;

    fn path(&self) -> Cow<str> {
        format!("/manga/{:x}", self.id).into()
    }

    fn method(&self) -> Method {
        Method::PUT
    }

    fn require_auth(&self) -> bool {
        true
    }

    fn body(&self) -> Option<&Self::Body> {
        Some(&self.request)
    }
}

/// View manga
///
/// Call to `GET /manga/{id}`
pub struct ViewManga<'a> {
    pub id: &'a Uuid,
}

impl Endpoint for ViewManga<'_> {
    type Query = ();
    type Body = ();
    type Response = MangaResponse;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/manga/{:x}", self.id))
    }
}

/// Delete manga (requires authentication)
///
/// Call to `DELETE /manga/{id}`
pub struct DeleteManga<'a> {
    pub id: &'a Uuid,
}

impl Endpoint for DeleteManga<'_> {
    type Query = ();
    type Body = ();
    type Response = Result<NoData>;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/manga/{:x}", self.id))
    }

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn require_auth(&self) -> bool {
        true
    }
}

/// Follow manga (requires authentication)
///
/// Call to `POST /manga/{id}/follow`
pub struct FollowManga<'a> {
    pub id: &'a Uuid,
}

impl Endpoint for FollowManga<'_> {
    type Query = ();
    type Body = ();
    type Response = Result<NoData>;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/manga/{:x}/follow", self.id))
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn require_auth(&self) -> bool {
        true
    }
}

/// Unfollow manga (requires authentication)
///
/// Call to `DELETE /manga/{id}/follow`
pub struct UnfollowManga<'a> {
    pub id: &'a Uuid,
}

impl Endpoint for UnfollowManga<'_> {
    type Query = ();
    type Body = ();
    type Response = Result<NoData>;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/manga/{:x}/follow", self.id))
    }

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn require_auth(&self) -> bool {
        true
    }
}

/// Get logged user followed manga list (requires authentication)
///
/// Call to `GET /usr/follows/manga`
pub struct FollowedMangaList<'a> {
    pub query: &'a PaginationQuery,
}

impl Endpoint for FollowedMangaList<'_> {
    type Query = PaginationQuery;
    type Body = ();
    type Response = MangaList;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("/user/follows/manga")
    }

    fn require_auth(&self) -> bool {
        true
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.query)
    }
}

/// Get a random manga
///
/// Call to `GET /manga/random`
pub struct RandomManga;

impl Endpoint for RandomManga {
    type Query = ();
    type Body = ();
    type Response = MangaResponse;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("/manga/random")
    }
}

/// Global tag list
///
/// Call to `GET /manga/tags`
pub struct TagList;

impl Endpoint for TagList {
    type Query = ();
    type Body = ();
    type Response = types::TagList;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("/manga/tag")
    }
}

impl Client {
    /// List manga.
    pub async fn list_manga(&self, query: &MangaQuery) -> Result<MangaList> {
        self.send_request(&ListManga { query }).await
    }

    /// Create a manga.
    ///
    /// Requires auth.
    pub async fn create_manga(&self, request: &MangaRequest) -> MangaResponse {
        self.send_request(&CreateManga { request }).await?
    }

    /// Update a manga.
    ///
    /// Requires auth.
    pub async fn update_manga(&self, id: &Uuid, request: &MangaRequest) -> MangaResponse {
        self.send_request(&UpdateManga { id, request }).await?
    }

    /// View a single manga.
    pub async fn view_manga(&self, id: &Uuid) -> MangaResponse {
        self.send_request(&ViewManga { id }).await?
    }

    /// Delete a manga.
    ///
    /// Requires auth.
    pub async fn delete_manga(&self, id: &Uuid) -> Result<()> {
        self.send_request(&DeleteManga { id }).await.map(|_| ())
    }

    pub async fn follow_manga(&self, id: &Uuid) -> Result<()> {
        self.send_request(&FollowManga { id }).await.map(|_| ())
    }

    pub async fn unfollow_manga(&self, id: &Uuid) -> Result<()> {
        self.send_request(&UnfollowManga { id }).await.map(|_| ())
    }

    pub async fn manga_reading_status(&self, id: &Uuid) -> Result<MangaReadingStatus> {
        self.send_request(&GetMangaStatus { id })
            .await?
            .map(|s| s.status)
    }

    pub async fn update_manga_reading_status(
        &self,
        id: &Uuid,
        status: MangaReadingStatus,
    ) -> Result<()> {
        self.send_request(&UpdateMangaStatus { id, status })
            .await?
            .map(|_| ())
    }

    pub async fn manga_feed(&self, manga_id: &Uuid, query: &MangaFeedQuery) -> Result<ChapterList> {
        self.send_request(&MangaFeed { manga_id, query }).await
    }

    /// Get logged User followed Manga feed
    pub async fn followed_manga_feed(&self, query: &MangaFeedQuery) -> Result<ChapterList> {
        self.send_request(&FollowedMangaFeed { query }).await
    }

    pub async fn followed_manga_list(&self, query: &PaginationQuery) -> Result<MangaList> {
        self.send_request(&FollowedMangaList { query }).await
    }

    /// Get a random Manga
    pub async fn random_manga(&self) -> MangaResponse {
        self.send_request(&RandomManga).await?
    }

    pub async fn tag_list(&self) -> Result<types::TagList> {
        self.send_request(&TagList).await
    }

    /// A list of chapter ids that are marked as read for the given manga ids
    pub async fn manga_read_markers(&self, manga_ids: &[&Uuid]) -> Result<ApiData<Vec<Uuid>>> {
        self.send_request(&MangaReadMarkers {
            ids: manga_ids.into(),
            // grouped: None,
        })
        .await?
    }

    /// Get all Manga reading status for logged User
    pub async fn all_manga_reading_status(
        &self,
        status: Option<MangaReadingStatus>,
    ) -> Result<HashMap<Uuid, MangaReadingStatus>> {
        self.send_request(&AllMangaStatus { status })
            .await?
            .map(|s| s.statuses)
    }
}
