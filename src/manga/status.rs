use std::borrow::Cow;

use reqwest::Method;
use serde::Serialize;
use uuid::Uuid;

use super::{MangaReadingStatus, MangaReadingStatusBody, MangaReadingStatuses};
use crate::{ApiData, Endpoint, NoData, Result};

/// Get manga reading status (requires authentication)
///
/// Call to `GET /manga/{id}/status`
pub struct GetMangaStatus<'a> {
    pub id: &'a Uuid,
}

impl Endpoint for GetMangaStatus<'_> {
    type Query = ();
    type Body = ();
    type Response = Result<MangaReadingStatusBody>;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/manga/{:x}/status", self.id))
    }

    fn require_auth(&self) -> bool {
        true
    }
}

/// Update manga reading status (requires authentication)
///
/// Call to `POST /manga/{id}/status`
#[derive(Serialize)]
pub struct UpdateMangaStatus<'a> {
    #[serde(skip)]
    pub id: &'a Uuid,
    pub status: MangaReadingStatus,
}

impl Endpoint for UpdateMangaStatus<'_> {
    type Query = ();
    type Body = Self;
    type Response = Result<NoData>;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/manga/{:x}/status", self.id))
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn require_auth(&self) -> bool {
        true
    }

    fn body(&self) -> Option<&Self::Body> {
        Some(&self)
    }
}

/// Get all manga reading status for logged user (requires authentication)
///
/// Call to `GET /manga/status`
#[derive(Serialize)]
pub struct AllMangaStatus {
    pub status: Option<MangaReadingStatus>,
}

impl Endpoint for AllMangaStatus {
    type Query = Self;
    type Body = ();
    type Response = Result<MangaReadingStatuses>;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("/manga/status")
    }

    fn require_auth(&self) -> bool {
        true
    }
}

/// Manga read markers (requires authentication)
///
/// A list of chapter ids that are marked as read for the specified manga.
///
/// Call to `GET /manga/read`
#[derive(Serialize)]
pub struct MangaReadMarkers<'a> {
    pub ids: Vec<&'a Uuid>,
    // pub grouped: Option<bool>,
}

impl Endpoint for MangaReadMarkers<'_> {
    type Query = Self;
    type Body = ();
    type Response = Result<ApiData<Vec<Uuid>>>;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("/manga/read")
    }

    fn require_auth(&self) -> bool {
        true
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self)
    }
}
