//! Cover art

use std::borrow::Cow;

use derive_builder::Builder;
use reqwest::multipart::Form;
use reqwest::multipart::Part;
use reqwest::Method;
use serde::Serialize;
use uuid::Uuid;

use crate::common::Endpoint;
use crate::schema::{cover::*, NoData};
use crate::Client;
use crate::Result;

/// Cover art list
///
/// Call to `GET /cover`
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[builder(default, setter(strip_option))]
pub struct ListCovers<'a> {
    /// Page size (max 100)
    pub limit: Option<i32>,

    /// Page offset (max ~10k)
    pub offset: Option<i32>,

    /// Manga ids (max 100)
    #[builder(setter(each = "add_manga"))]
    pub manga: Vec<&'a Uuid>,

    /// Cover ids (max 100)
    #[serde(rename = "ids")]
    #[builder(setter(each = "add_cover"))]
    pub covers: Vec<&'a Uuid>,

    /// Uploader ids (max 100)
    #[builder(setter(each = "add_uploader"))]
    pub uploaders: Vec<&'a Uuid>,

    /// Sort order
    pub order: Option<CoverOrder>,
}

impl_endpoint! {
    GET "/cover",
    #[query] ListCovers<'_>,
    CoverList
}

/// Get cover
///
/// Call to `GET /cover/{cover_id}`
#[derive(Debug, Clone)]
pub struct GetCover<'a> {
    pub cover_id: &'a Uuid,
}

impl_endpoint! {
    GET ("/cover/{:x}", cover_id),
    #[no_data] GetCover<'_>,
    #[flatten_result] CoverResponse
}

/// Edit cover
///
/// Call to `PUT /cover/{cover_id}`
#[derive(Debug, Serialize, Clone)]
pub struct EditCover<'a> {
    /// Cover id
    #[serde(skip)]
    pub cover_id: &'a Uuid,

    /// Cover volume
    pub volume: Option<&'a str>,

    /// Cover description
    pub description: Option<&'a str>,

    /// Cover version
    pub version: i32,
}

impl_endpoint! {
    PUT ("/cover/{:x}", cover_id),
    #[body auth] EditCover<'_>,
    #[flatten_result] CoverResponse
}

/// Delete cover
///
/// Call to `DELETE /cover/{cover_id}`
pub struct DeleteCover<'a> {
    pub cover_id: &'a Uuid,
}

impl_endpoint! {
    DELETE ("/cover/{:x}", cover_id),
    #[no_data auth] DeleteCover<'_>,
    #[discard_result] Result<NoData>
}

/// Upload cover (requires authentication)
///
/// Call to `POST /cover/{manga_id}`
pub struct UploadCover<'a> {
    /// Manga id
    pub manga_id: &'a Uuid,

    /// Image bytes
    pub image_bytes: Cow<'static, [u8]>,
}

impl Endpoint for UploadCover<'_> {
    type Query = ();
    type Body = ();
    type Response = CoverResponse;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/cover/{:x}", self.manga_id))
    }
    fn method(&self) -> Method {
        Method::POST
    }

    fn require_auth(&self) -> bool {
        true
    }

    fn multipart(&self) -> Option<Form> {
        let part = Part::bytes(self.image_bytes.clone());
        Some(Form::new().part("file", part))
    }
}

impl UploadCover<'_> {
    pub async fn send(&self, client: &Client) -> CoverResponse {
        client.send_request(self).await?
    }
}
