use crate::{
    common::{deserialize_null_default, ApiObject, LocalizedString, Results},
    errors::Result,
    ApiData, Client, NoData, OrderType, PaginationQuery, UrlSerdeQS,
};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAttributes {
    pub username: String,
    pub version: i32,
}

pub type User = ApiObject<UserAttributes>;
pub type UserResponse = Result<ApiData<User>>;

impl Client {
    /// Create an account.
    pub async fn create_account(
        &self,
        username: &str,
        password: &str,
        email: &str,
    ) -> UserResponse {
        let endpoint = self.base_url.join("/account/create")?;
        let res = self
            .http
            .post(endpoint)
            .json(&serde_json::json!({
                "username": username,
                "password": password,
                "email": email
            }))
            .send()
            .await?;

        Self::json_api_result(res).await
    }

    /// Activate an account.
    pub async fn activate_account(&self, code: &str) -> Result<()> {
        let endpoint = self.base_url.join(&format!("/account/activate/{}", code))?;

        let res = self.http.get(endpoint).send().await?;

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }

    /// Resent the activation code to the email.
    pub async fn resend_activation_code(&self, email: &str) -> Result<()> {
        let endpoint = self.base_url.join("/account/activate/resend")?;
        let res = self
            .http
            .post(endpoint)
            .json(&serde_json::json!({ "email": email }))
            .send()
            .await?;

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }

    /// Recover an account.
    pub async fn recover_account(&self, email: &str) -> Result<()> {
        let endpoint = self.base_url.join("/account/recover")?;
        let res = self
            .http
            .post(endpoint)
            .json(&serde_json::json!({ "email": email }))
            .send()
            .await?;

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }

    /// Complete the recovery of an account.
    pub async fn complete_account_recover(&self, code: &str, new_password: &str) -> Result<()> {
        let endpoint = self.base_url.join(&format!("/account/recover/{}", code))?;
        let res = self
            .http
            .post(endpoint)
            .json(&serde_json::json!({ "newPassword": new_password }))
            .send()
            .await?;

        Self::json_api_result::<NoData>(res).await?;
        Ok(())
    }
}
