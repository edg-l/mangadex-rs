//! Solve captcha

use serde::Serialize;

use crate::{schema::NoData, Result};

/// Solve captcha
///
/// Call to `POST /captcha/solve`
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SolveCaptcha<'a> {
    pub captcha_challenge: &'a str,
}

impl_endpoint! {
    POST "/captcha/solve",
    #[body] SolveCaptcha<'_>,
    #[discard_result] Result<NoData>
}
