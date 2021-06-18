use serde::{Deserialize, Serialize};

/// Tokens returned on login.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthTokens {
    /// A token that lives for 15 minutes.
    pub session: String,
    /// A token that lives for 1 month. Allows getting another refresh token.
    pub refresh: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginResponse {
    #[serde(rename = "token")]
    pub tokens: AuthTokens,
}

/// Response when checking a token.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckTokenResponse {
    pub is_authenticated: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

/// The response when refreshing the session token.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenResponse {
    #[serde(rename = "token")]
    pub tokens: AuthTokens,
    pub message: Option<String>,
}
