use std::collections::HashMap;

use serde::Serialize;
use uuid::Uuid;

use super::{MangaReadingStatus, MangaReadingStatusBody, MangaReadingStatuses};
use crate::{ApiData, Client, NoData, Result};

/// Get manga reading status (requires authentication)
///
/// Call to `GET /manga/{id}/status`
#[derive(Debug, Clone)]
pub struct GetMangaStatus<'a> {
    pub id: &'a Uuid,
}

impl_endpoint! {
    GET ("/manga/{:x}/status", id),
    #[no_data auth] GetMangaStatus<'_>,
    #[no_send] Result<MangaReadingStatusBody>
}

impl GetMangaStatus<'_> {
    pub async fn send(&self, client: &Client) -> Result<MangaReadingStatus> {
        client.send_request(self).await?.map(|r| r.status)
    }
}

/// Update manga reading status (requires authentication)
///
/// Call to `POST /manga/{id}/status`
#[derive(Debug, Serialize, Clone)]
pub struct UpdateMangaStatus<'a> {
    #[serde(skip)]
    pub id: &'a Uuid,
    pub status: MangaReadingStatus,
}

impl_endpoint! {
    POST ("/manga/{:x}/status", id),
    #[body auth] UpdateMangaStatus<'_>,
    #[discard_result] Result<NoData>
}

/// Get all manga reading status for logged user (requires authentication)
///
/// Call to `GET /manga/status`
#[derive(Serialize)]
pub struct AllMangaStatus {
    pub status: Option<MangaReadingStatus>,
}

impl_endpoint! {
    GET "/manga/status",
    #[query auth] AllMangaStatus,
    #[no_send] Result<MangaReadingStatuses>
}

impl AllMangaStatus {
    pub async fn send(&self, client: &Client) -> Result<HashMap<Uuid, MangaReadingStatus>> {
        client.send_request(self).await?.map(|r| r.statuses)
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

impl_endpoint! {
    GET "/manga/read",
    #[query auth] MangaReadMarkers<'_>,
    #[flatten_result] Result<ApiData<Vec<Uuid>>>
}
