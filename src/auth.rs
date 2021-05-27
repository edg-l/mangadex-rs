use super::Client;
use crate::common::*;
use crate::errors::*;
use serde::{Deserialize, Serialize};

/// Tokens returned on login.
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Serialize)]
struct LoginRequest<'a> {
    username: &'a str,
    password: &'a str,
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
#[derive(Serialize)]
struct RefreshTokenRequest<'a> {
    /// This token must be the refresh token.
    pub token: &'a str,
}

/// The response when refreshing the session token.
#[derive(Debug, Deserialize, Clone)]
pub struct RefreshTokenResponse {
    pub token: AuthTokens,
    pub message: Option<String>,
}

impl Client {
    /// Login endpoint
    ///
    /// * `username` - Should be between [1, 64] characters.
    /// * `password` - Should be between [8, 1024] characters.
    pub async fn login(&mut self, username: &str, password: &str) -> Result<AuthTokens> {
        let endpoint = self.base_url.join("/auth/login")?;
        let request = LoginRequest { username, password };

        let res = self.http.post(endpoint).json(&request).send().await?;
        let res = Self::json_api_result::<LoginResponse>(res).await?;

        self.set_tokens(Some(res.token.clone()));
        Ok(res.token)
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
            token: &tokens.refresh,
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
    use crate::tests::*;

    #[tokio::test]
    #[ignore = "require api auth"]
    async fn endpoint_login() {
        // Don't login again to avoid ratelimits
        if has_tokens() {
            return;
        }
        let auth_details = get_auth_details();

        let mut client = Client::default();
        let res = client
            .login(&auth_details.0, &auth_details.1)
            .await
            .unwrap();

        assert!(!res.refresh.is_empty());
        assert!(!res.session.is_empty());

        client.check_token().await.unwrap();
    }

    #[tokio::test]
    #[ignore = "require api auth"]
    async fn endpoint_check_token() {
        if !has_tokens() {
            return;
        }
        let tokens = get_tokens();

        let mut client = Client::default();
        client.set_tokens(Some(tokens));
        client.check_token().await.unwrap();
    }

    #[tokio::test]
    #[ignore = "require api auth"]
    async fn endpoint_refresh_token() {
        if !has_tokens() {
            return;
        }
        let tokens = get_tokens();

        let mut client = Client::default();
        client.set_tokens(Some(tokens));
        client.refresh_token().await.unwrap();
    }
}
