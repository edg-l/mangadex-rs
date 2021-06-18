//! Account management

use crate::model::user::*;
use crate::model::NoData;

use crate::Result;
use serde::Serialize;

/// Create account
///
/// Call to `POST /account/create`
#[derive(Debug, Serialize, Clone)]
pub struct CreateAccount<'a> {
    /// Username (length 1 to 64)
    pub username: &'a str,
    /// Password (length 8 to 1024)
    pub password: &'a str,
    /// Email
    pub email: &'a str,
}

impl_endpoint! {
    POST "/account/create",
    #[body] CreateAccount<'_>,
    #[flatten_result] UserResponse
}

/// Activate account
///
/// Call to `GET /account/activate/{code}`
#[derive(Debug, Clone)]
pub struct ActivateAccount<'a> {
    /// Account activation code
    pub code: &'a str,
}

impl_endpoint! {
    GET ("/account/activate/{}", code),
    #[no_data] ActivateAccount<'_>,
    #[discard_result] Result<NoData>
}

/// Resend activation code
///
/// Call to `POST /account/activate/resend`
#[derive(Debug, Serialize, Clone)]
pub struct ResendActivationCode<'a> {
    /// Email
    pub email: &'a str,
}

impl_endpoint! {
    POST "/account/activate/resend",
    #[body] ResendActivationCode<'_>,
    #[discard_result] Result<NoData>
}

/// Recover account
///
/// Call to `POST /account/recover`
#[derive(Debug, Serialize, Clone)]
pub struct RecoverAccount<'a> {
    /// Email
    pub email: &'a str,
}

impl_endpoint! {
    POST "/account/recover",
    #[body] RecoverAccount<'_>,
    #[discard_result] Result<NoData>
}

/// Complete account recover
///
/// Call to `POST /account/recover`
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompleteAccountRecover<'a> {
    /// Account recovery code
    #[serde(skip)]
    code: &'a str,
    /// New password (length 8 to 1024)
    new_password: &'a str,
}

impl_endpoint! {
    POST ("/account/recover/{}", code),
    #[body] CompleteAccountRecover<'_>,
    #[discard_result] Result<NoData>
}
