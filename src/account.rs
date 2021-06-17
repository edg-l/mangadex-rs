use crate::{common::ApiObject, errors::Result, ApiData, NoData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAttributes {
    pub username: String,
    pub version: i32,
}

pub type User = ApiObject<UserAttributes>;
pub type UserResponse = Result<ApiData<User>>;

/// Create account
///
/// Call to `POST /account/create`
#[derive(Debug, Serialize, Clone)]
pub struct CreateAccountReq<'a> {
    username: &'a str,
    password: &'a str,
    email: &'a str,
}

impl_endpoint! {
    POST "/account/create",
    #[body] CreateAccountReq<'_>,
    #[flatten_result] UserResponse
}

/// Activate account
///
/// Call to `GET /account/activate/{code}`
#[derive(Debug, Clone)]
pub struct ActivateAccountReq<'a> {
    code: &'a str,
}

impl_endpoint! {
    GET ("/account/activate/{}", code),
    #[no_data] ActivateAccountReq<'_>,
    #[discard_result] Result<NoData>
}

/// Resend activation code
///
/// Call to `POST /account/activate/resend`
#[derive(Debug, Serialize, Clone)]
pub struct ResendActivationCodeReq<'a> {
    email: &'a str,
}

impl_endpoint! {
    POST "/account/activate/resend",
    #[body] ResendActivationCodeReq<'_>,
    #[discard_result] Result<NoData>
}

/// Recover account
///
/// Call to `POST /account/recover`
#[derive(Debug, Serialize, Clone)]
pub struct RecoverAccountReq<'a> {
    email: &'a str,
}

impl_endpoint! {
    POST "/account/recover",
    #[body] RecoverAccountReq<'_>,
    #[discard_result] Result<NoData>
}

/// Complete account recover
///
/// Call to `POST /account/recover`
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompleteAccountRecoverReq<'a> {
    #[serde(skip)]
    code: &'a str,
    new_password: &'a str,
}

impl_endpoint! {
    POST ("/account/recover/{}", code),
    #[body] CompleteAccountRecoverReq<'_>,
    #[discard_result] Result<NoData>
}
