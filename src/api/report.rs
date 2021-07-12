//! Reports

use crate::schema::report::*;
use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

/// List reports.
///
/// Call to `GET /reports/reasons/{category}`
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
pub struct ListReports {
    pub category: ReportCategory,
}

impl_endpoint! {
    GET ("/reports/reasons/{}", category),
    #[query] ListReports,
    ReportList
}

/// Create a new report (requires authentication)
///
/// Call to `POST /report`
#[derive(Debug, Serialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct CreateReport<'a> {
    pub category: ReportCategory,
    /// The reason.
    pub reason: &'a str,
    /// The object id.
    pub object_id: &'a Uuid,
    /// The details.
    pub details: &'a str,
}

impl_endpoint! {
    POST "/report",
    #[body auth] CreateReport<'_>,
    #[flatten_result] ReportResponse
}
