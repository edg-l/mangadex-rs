use chrono::{DateTime, Utc};
use jwt::Token;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct RefreshTokenClaims {
    /// Type
    pub typ: String,
    /// Issuer
    pub iss: String,
    /// Audience
    pub aud: String,
    /// Issued at
    pub iat: DateTime<Utc>,
    /// Not before
    pub nbf: DateTime<Utc>,
    /// Expiration time.
    pub exp: DateTime<Utc>,
    /// User id
    pub uid: Uuid,
    /// Session id
    pub sid: Uuid,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct SessionTokenClaims {
    /// Type
    pub typ: String,
    /// Issuer
    pub iss: String,
    /// Audience
    pub aud: String,
    /// Issued at
    pub iat: DateTime<Utc>,
    /// Not before
    pub nbf: DateTime<Utc>,
    /// Expiration time.
    pub exp: DateTime<Utc>,
    /// User id
    pub uid: Uuid,
    /// Session id
    pub sid: Uuid,
    /// User roles
    // TODO: De-stringify the roles.
    pub rol: Vec<String>,
    /// User permissions
    // TODO: De-stringify the permissions.
    pub prm: Vec<String>,
}

pub fn deserialize_jwt<T>(
    token: &'_ str,
) -> std::result::Result<jwt::Token<jwt::Header, T, jwt::Unverified<'_>>, jwt::Error>
where
    T: serde::de::DeserializeOwned,
{
    let token: Token<jwt::Header, T, _> = Token::parse_unverified(token)?;
    Ok(token)
}
