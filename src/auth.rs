use super::Client;
use crate::common::*;
use crate::errors::*;
use serde::{Deserialize, Serialize};

/// Tokens returned on login.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthTokens {
    /// A token that lives for 15 minutes.
    pub session: String,
    /// A token that lives for 1 month. Allows getting another refresh token.
    pub refresh: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    pub token: AuthTokens,
}

/// Request payload to login.
#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

/// Response when checking a token.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckTokenResponse {
    pub is_authenticated: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

/// Request payload to refresh the session token.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct RefreshTokenRequest {
    /// This token must be the refresh token.
    pub token: String,
}

/// The response when refreshing the session token.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenResponse {
    pub token: AuthTokens,
    pub message: Option<String>,
}

impl Client {
    /// Login endpoint
    ///
    /// * `username` - Should be between [1, 64] characters.
    /// * `password` - Should be between [8, 1024] characters.
    pub async fn login(&mut self, username: &str, password: &str) -> Result<&AuthTokens> {
        let endpoint = self.base_url.join("/auth/login")?;
        let request = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        let res = self.http.post(endpoint).json(&request).send().await?;
        let res = Self::json_api_result::<LoginResponse>(res).await?;

        self.set_tokens(Some(res.token));
        Ok(self.get_tokens().unwrap())
    }

    /// Get the tokens used for authentication
    pub fn get_tokens(&self) -> Option<&AuthTokens> {
        self.tokens.as_ref()
    }

    /// Set the tokens used for authentication.
    pub fn set_tokens(&mut self, tokens: Option<AuthTokens>) {
        self.tokens = tokens;
    }

    /// Convenience method to be used with ?.
    pub(crate) fn require_tokens(&self) -> Result<&AuthTokens> {
        self.tokens.as_ref().ok_or(Errors::MissingTokens)
    }

    /// Check token endpoint
    pub async fn check_token(&self) -> Result<CheckTokenResponse> {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/auth/check")?;

        let res = self
            .http
            .get(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;

        Self::json_api_result(res).await
    }

    /// Logout endpoint
    pub async fn logout(&mut self) -> Result<()> {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/auth/logout")?;

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;

        Self::json_api_result::<NoData>(res).await?;
        self.set_tokens(None);
        Ok(())
    }

    /// Refresh token endpoint
    pub async fn refresh_token(&mut self) -> Result<RefreshTokenResponse> {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/auth/refresh")?;

        let request = RefreshTokenRequest {
            token: tokens.refresh.to_string(),
        };

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .json(&request)
            .send()
            .await?;

        let res = Self::json_api_result::<RefreshTokenResponse>(res).await?;
        self.set_tokens(Some(res.token.clone()));
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use httpmock::Method::POST;
    use httpmock::MockServer;
    use serde_json::json;

    #[tokio::test]
    async fn login_ok() -> anyhow::Result<()> {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(POST)
                    .path("/auth/login")
                    .header("Content-Type", "application/json")
                    .json_body(json!({"username": "test", "password": "hunter1"}));
                then.status(200)
                    .header("Content-Type", "application/json")
                    .json_body(json!({
                        "result": "ok",
                        "token": {
                            "session": "sessiontoken",
                            "refresh": "refreshtoken",
                        }
                    }));
            })
            .await;

        let mut client = Client::new(&server.base_url())?;

        let tokens = client.login("test", "hunter1").await?;

        mock.assert_async().await;
        assert_eq!(tokens.session.as_str(), "sessiontoken");
        assert_eq!(tokens.refresh.as_str(), "refreshtoken");

        Ok(())
    }

    #[tokio::test]
    async fn login_err_400() -> anyhow::Result<()> {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(POST)
                    .path("/auth/login")
                    .header("Content-Type", "application/json")
                    .json_body(json!({"username": "test", "password": "hunter1"}));
                then.status(400)
                    .header("Content-Type", "application/json")
                    .json_body(json!({
                        "result": "error",
                        "errors": [],
                    }));
            })
            .await;

        let mut client = Client::new(&server.base_url())?;

        let errors = client
            .login("test", "hunter1")
            .await
            .expect_err("should return an error");

        mock.assert_async().await;
        assert_matches!(errors, Errors::HttpWithBody(x) if x.errors.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn login_err_401() -> anyhow::Result<()> {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(POST)
                    .path("/auth/login")
                    .header("Content-Type", "application/json")
                    .json_body(json!({"username": "test", "password": "hunter1"}));
                then.status(401);
            })
            .await;

        let mut client = Client::new(&server.base_url())?;

        let errors = client
            .login("test", "hunter1")
            .await
            .expect_err("should return an error");

        mock.assert_async().await;
        assert_matches!(errors, Errors::Http(_));

        Ok(())
    }
}
