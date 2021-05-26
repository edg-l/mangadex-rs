use super::Client;
use crate::common::*;
use crate::errors::*;
use serde::{Deserialize, Serialize};

// Tokens returned on login.
#[derive(Debug, Deserialize, Clone)]
pub struct AuthTokens {
    /// A token that lives for 15 minutes.
    pub session: String,
    /// A token that lives for 1 month. Allows getting another refresh token.
    pub refresh: String,
}

/// The response returned on [Client::login].
#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub result: ApiResult,
    pub token: AuthTokens,
}

/// Request payload to login.
#[derive(Debug, Serialize)]
struct LoginRequest<'a> {
    username: &'a str,
    password: &'a str,
}

/// Response when checking a token.
#[derive(Debug, Deserialize)]
pub struct CheckTokenResponse {
    pub result: ApiResult,
    #[serde(rename(deserialize = "isAuthenticated"))]
    pub is_authenticated: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

/// Request payload to refresh the session token.
#[derive(Debug, Serialize)]
struct RefreshTokenRequest<'a> {
    /// This token must be the refresh token.
    pub token: &'a str,
}

/// The response when refreshing the session token.
#[derive(Debug, Deserialize)]
pub struct RefreshTokenResponse {
    pub result: ApiResult,
    pub token: AuthTokens,
    pub message: Option<String>,
}

impl Client {
    /// Login endpoint
    ///
    /// * `username` - Should be between [1, 64] characters.
    /// * `password` - Should be between [8, 1024] characters.
    pub async fn login(&mut self, username: &str, password: &str) -> Result<LoginResponse> {
        let endpoint = self.base_url.join("/auth/login")?;

        let request = LoginRequest { username, password };

        let res = self.http.post(endpoint).json(&request).send().await?;
        let login = Self::deserialize_response::<LoginResponse, ApiErrors>(res).await?;

        self.set_tokens(&login.token);

        Ok(login)
    }

    /// Set the tokens used for authentication.
    pub fn set_tokens(&mut self, tokens: &AuthTokens) {
        self.tokens = Some(tokens.clone());
    }

    /// Clears the stored tokens, like a logout.
    pub fn clear_tokens(&mut self) {
        self.tokens = None;
    }

    /// Convenience method to be used with ?.
    pub(crate) fn require_tokens(&self) -> Result<&AuthTokens> {
        match &self.tokens {
            Some(tokens) => Ok(tokens),
            None => Err(Errors::MissingTokens),
        }
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

        let res = Self::deserialize_response::<CheckTokenResponse, ApiErrors>(res).await?;

        Ok(res)
    }

    /// Logout endpoint
    pub async fn logout(&mut self) -> Result<SimpleApiResponse> {
        let tokens = self.require_tokens()?;
        let endpoint = self.base_url.join("/auth/logout")?;

        let res = self
            .http
            .post(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;
        let res = Self::deserialize_response::<SimpleApiResponse, ApiErrors>(res).await?;
        self.clear_tokens();

        Ok(res)
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
        let res = Self::deserialize_response::<RefreshTokenResponse, ApiErrors>(res).await?;
        self.set_tokens(&res.token);

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    #[ignore = "require api auth"]
    async fn endpoint_login() {
        // Don't login again to avoid ratelimits
        if has_tokens() {
            return;
        }
        let auth_details = get_auth_details();

        let mut client = Client::new().unwrap();
        let res = client
            .login(&auth_details.0, &auth_details.1)
            .await
            .unwrap();

        assert!(!res.token.refresh.is_empty());
        assert!(!res.token.session.is_empty());

        println!("{:#?}", res.token);

        let res = client.check_token().await.unwrap();
        assert_eq!(ApiResult::Ok, res.result);
    }

    #[tokio::test]
    #[ignore = "require api auth"]
    async fn endpoint_check_token() {
        if !has_tokens() {
            return;
        }
        let tokens = get_tokens();

        let mut client = Client::new().unwrap();
        client.set_tokens(&tokens);
        let res = client.check_token().await.unwrap();

        assert_eq!(ApiResult::Ok, res.result);

        println!("{:#?}", res);
    }

    #[tokio::test]
    #[ignore = "require api auth"]
    async fn endpoint_refresh_token() {
        if !has_tokens() {
            return;
        }
        let tokens = get_tokens();

        let mut client = Client::new().unwrap();
        client.set_tokens(&tokens);
        let res = client.refresh_token().await.unwrap();

        assert_eq!(ApiResult::Ok, res.result);

        println!("{:#?}", res);
    }
}
