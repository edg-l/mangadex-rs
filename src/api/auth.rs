//! User authentication

use crate::schema::auth::*;
use crate::schema::NoData;
use crate::Result;

use serde::Serialize;

/// Login with given username and password
///
/// This struct only does the API call.
/// In order to update the tokens used by the [`Client`][crate::Client], use the
/// [`Client::login()`][crate::Client::login()] method.
///
/// Call to `POST /auth/login`
#[derive(Debug, Serialize, Clone)]
pub struct Login<'a> {
    /// Username (length 1 to 64)
    pub username: &'a str,
    /// Password (length 8 to 1024)
    pub password: &'a str,
}

impl_endpoint! {
    POST "/auth/login",
    #[body] Login<'_>,
    #[flatten_result] Result<LoginResponse>
}

/// Check permissions for the logged user
///
/// Call to `GET /auth/check`
#[derive(Debug, Clone)]
pub struct CheckToken;

impl_endpoint! {
    GET "/auth/check",
    #[no_data auth] CheckToken,
    #[flatten_result] Result<CheckTokenResponse>
}

/// Send a logout request for the logged user
///
/// This struct only does the API call.
/// In order to update the tokens used by the [`Client`][crate::Client], use the
/// [`Client::logout()`][crate::Client::logout()] method.
///
/// Call to `POST /auth/logout`
#[derive(Debug, Clone)]
pub struct Logout;

impl_endpoint! {
    POST "/auth/logout",
    #[no_data auth] Logout,
    #[discard_result] Result<NoData>
}

/// Get a new session and refresh token
///
/// This struct only does the API call.
/// In order to update the tokens used by the [`Client`][crate::Client], use the
/// [`Client::refresh_tokens()`][crate::Client::refresh_tokens()] method.
///
/// Call to `POST /auth/refresh`
#[derive(Debug, Serialize, Clone)]
pub struct RefreshToken<'a> {
    /// Refresh token
    #[serde(rename = "token")]
    pub refresh_token: &'a str,
}

impl_endpoint! {
    POST "/auth/refresh",
    #[body] RefreshToken<'_>,
    #[flatten_result] Result<RefreshTokenResponse>
}

#[cfg(test)]
mod tests {
    use crate::errors::Errors;
    use crate::Client;

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
        assert_matches!(errors, Errors::Api(x) if x.errors.is_empty());

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

        let info = CheckToken.send(&client).await?;

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

        assert_matches!(errors, Errors::Api(errs) if errs.errors.len() == 1usize => {
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
                    .header("Content-Type", "application/json");
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

        let res = client.refresh_tokens().await?;

        mock.assert_async().await;
        assert_eq!(client.get_tokens().is_some(), true);
        let tokens = client.get_tokens().unwrap();

        assert_eq!(tokens.session.as_str(), "sessiontoken2");
        assert_eq!(tokens.refresh.as_str(), "refreshtoken2");
        assert_eq!(res.tokens.session.as_str(), "sessiontoken2");
        assert_eq!(res.tokens.refresh.as_str(), "refreshtoken2");

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
                            ;// .header("Authorization", "Bearer sessiontoken");
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

                let errors = client.refresh_tokens().await.expect_err("expected error");
                mock.assert_async().await;

                assert_matches!(errors, Errors::Api(errs) if errs.errors.len() == 1usize => {
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
