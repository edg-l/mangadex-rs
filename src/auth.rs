use super::Client;
use crate::common::*;
use crate::errors::*;
use serde::{Deserialize, Serialize};

// Tokens returned on login.
#[derive(Debug, Deserialize, Clone)]
pub struct AuthTokens {
    pub session: String,
    pub refresh: String,
}

/// The response returned on [Client::login].
#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub result: ApiResult,
    pub token: AuthTokens,
}

#[derive(Debug, Serialize)]
struct LoginRequest<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct CheckTokenResponse {
    pub result: ApiResult,
    #[serde(rename(deserialize = "isAuthenticated"))]
    pub is_authenticated: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
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

        let res = res.json::<LoginResponse>().await?;

        self.set_tokens(&res.token);

        Ok(res)
    }

    // Set the tokens used for authentication.
    pub fn set_tokens(&mut self, tokens: &AuthTokens) {
        self.tokens = Some(tokens.clone());
    }

    // Convenience method to be used with ?.
    fn require_tokens(&self) -> Result<&AuthTokens> {
        match &self.tokens {
            Some(tokens) => Ok(tokens),
            None => Err(Errors::MissingTokens),
        }
    }

    pub async fn check_token(&self) -> Result<CheckTokenResponse> {
        let tokens = self.require_tokens()?;

        let endpoint = self.base_url.join("/auth/check")?;

        let res = self
            .http
            .get(endpoint)
            .bearer_auth(&tokens.session)
            .send()
            .await?;

        Ok(res.json::<CheckTokenResponse>().await?)
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

        assert_eq!(ApiResult::OK, res.result);

        println!("{:#?}", res);
    }
}
