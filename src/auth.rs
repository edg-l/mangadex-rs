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
    use httpmock::Method::*;
    use httpmock::MockServer;
    use pretty_assertions::assert_eq;
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

    #[tokio::test]
    async fn check_token() -> anyhow::Result<()> {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path("/auth/check")
                    .header("Authorization", "Bearer sessiontoken");
                then.status(200)
                    .header("Content-Type", "application/json")
                    .json_body(json!({
                        "result": "ok",
                        "isAuthenticated": true,
                        "roles": [
                            "ROLE_MEMBER",
                            "IS_JWT_AUTHENTICATED",
                            "IS_AUTHENTICATED_FULLY",
                            "IS_AUTHENTICATED_ANONYMOUSLY",
                            "IS_AUTHENTICATED_REMEMBERED"
                        ],
                        "permissions": [
                            "user.list",
                            "manga.view",
                            "chapter.view",
                            "author.view",
                            "scanlation_group.view",
                            "cover.view",
                            "manga.list",
                            "chapter.list",
                            "author.list",
                            "scanlation_group.list",
                            "cover.list"
                        ],
                    }));
            })
            .await;

        let mut client = Client::new(&server.base_url())?;

        client.set_tokens(Some(AuthTokens {
            session: "sessiontoken".to_string(),
            refresh: "refreshtoken".to_string(),
        }));

        assert_eq!(client.get_tokens().is_some(), true);

        let info = client.check_token().await?;

        mock.assert_async().await;
        assert_eq!(info.is_authenticated, true);
        assert_eq!(info.permissions.len(), 11usize);
        assert_eq!(info.roles.len(), 5usize);

        Ok(())
    }

    #[tokio::test]
    async fn logout() -> anyhow::Result<()> {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(POST)
                    .path("/auth/logout")
                    .header("Authorization", "Bearer sessiontoken");
                then.status(200)
                    .header("Content-Type", "application/json")
                    .json_body(json!({
                        "result": "ok",
                    }));
            })
            .await;

        let mut client = Client::new(&server.base_url())?;

        client.set_tokens(Some(AuthTokens {
            session: "sessiontoken".to_string(),
            refresh: "refreshtoken".to_string(),
        }));

        assert_eq!(client.get_tokens().is_some(), true);

        client.logout().await?;

        mock.assert_async().await;
        assert_eq!(client.get_tokens().is_none(), true);

        Ok(())
    }

    #[tokio::test]
    async fn logout_503() -> anyhow::Result<()> {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(POST)
                    .path("/auth/logout")
                    .header("Authorization", "Bearer sessiontoken");
                then.status(503)
                    .header("Content-Type", "application/json")
                    .json_body(json!({
                        "result": "error",
                        "errors": [{
                            "id": "5e50fc7b-e185-45b1-a692-58e8091b22d2",
                            "title": "The service is unavailable",
                            "status": 503,
                            "detail": "Servers are burning",
                        }],
                    }));
            })
            .await;

        let mut client = Client::new(&server.base_url())?;

        client.set_tokens(Some(AuthTokens {
            session: "sessiontoken".to_string(),
            refresh: "refreshtoken".to_string(),
        }));

        assert_eq!(client.get_tokens().is_some(), true);

        let errors = client.logout().await.expect_err("expected error");

        mock.assert_async().await;

        assert_matches!(errors, Errors::HttpWithBody(errs) if errs.errors.len() == 1usize => {
            let error = errs.errors.get(0).unwrap();
            assert_eq!(error.id, uuid::Uuid::parse_str("5e50fc7b-e185-45b1-a692-58e8091b22d2")?);
            assert_eq!(error.title.as_deref(), Some("The service is unavailable"));
            assert_eq!(error.detail.as_deref(), Some("Servers are burning"));
            assert_eq!(error.status, 503);
        });

        Ok(())
    }

    #[tokio::test]
    async fn refresh_token() -> anyhow::Result<()> {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(POST)
                    .path("/auth/refresh")
                    .header("Content-Type", "application/json")
                    .header("Authorization", "Bearer sessiontoken");
                then.status(200)
                    .header("Content-Type", "application/json")
                    .json_body(json!({
                        "result": "ok",
                        "token": {
                            "session": "sessiontoken2",
                            "refresh": "refreshtoken2",
                        },
                        "message": "Token refreshed!",
                    }));
            })
            .await;

        let mut client = Client::new(&server.base_url())?;

        client.set_tokens(Some(AuthTokens {
            session: "sessiontoken".to_string(),
            refresh: "refreshtoken".to_string(),
        }));

        assert_eq!(client.get_tokens().is_some(), true);

        let res = client.refresh_token().await?;

        mock.assert_async().await;
        assert_eq!(client.get_tokens().is_some(), true);
        let tokens = client.get_tokens().unwrap();

        assert_eq!(tokens.session.as_str(), "sessiontoken2");
        assert_eq!(tokens.refresh.as_str(), "refreshtoken2");
        assert_eq!(res.token.session.as_str(), "sessiontoken2");
        assert_eq!(res.token.refresh.as_str(), "refreshtoken2");

        Ok(())
    }

    macro_rules! impl_refresh_token_err {
        ($fn_name:ident, $code:literal) => {
            #[tokio::test]
            async fn $fn_name() -> anyhow::Result<()> {
                let server = MockServer::start_async().await;

                let mock = server
                    .mock_async(|when, then| {
                        when.method(POST)
                            .path("/auth/refresh")
                            .header("Content-Type", "application/json")
                            .header("Authorization", "Bearer sessiontoken");
                        then.status($code)
                            .header("Content-Type", "application/json")
                            .json_body(json!({
                                "result": "error",
                                "errors": [{
                                    "id": "5e50fc7b-e185-45b1-a692-58e8091b22d2",
                                    "title": "Error title",
                                    "status": $code,
                                    "detail": "Error detail",
                                }],
                            }));
                    })
                    .await;

                let mut client = Client::new(&server.base_url())?;

                client.set_tokens(Some(AuthTokens {
                    session: "sessiontoken".to_string(),
                    refresh: "refreshtoken".to_string(),
                }));
                assert_eq!(client.get_tokens().is_some(), true);

                let errors = client.refresh_token().await.expect_err("expected error");
                mock.assert_async().await;

                assert_matches!(errors, Errors::HttpWithBody(errs) if errs.errors.len() == 1usize => {
                    let error = errs.errors.get(0).unwrap();
                    assert_eq!(error.id, uuid::Uuid::parse_str("5e50fc7b-e185-45b1-a692-58e8091b22d2")?);
                    assert_eq!(error.title.as_deref(), Some("Error title"));
                    assert_eq!(error.detail.as_deref(), Some("Error detail"));
                    assert_eq!(error.status, $code);
                });

                Ok(())
            }

        };
    }

    impl_refresh_token_err!(refresh_token_400, 400);

    impl_refresh_token_err!(refresh_token_401, 401);
    impl_refresh_token_err!(refresh_token_403, 403);
}
