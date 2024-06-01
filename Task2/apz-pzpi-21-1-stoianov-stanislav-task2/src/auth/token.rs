use core::fmt;

use anyhow::Context;
use jsonwebtoken::{get_current_timestamp, DecodingKey, EncodingKey, Header, Validation};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::JwtConfig;

use super::UserId;

pub type AccessToken = String;

pub type RefreshToken = String;

#[derive(Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct RefreshSecret(Uuid);

impl RefreshSecret {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessClaims {
    iat: u64,
    exp: u64,
    id: UserId,
}

#[tracing::instrument(skip(config), err(Debug))]
pub fn create_access_token(id: UserId, config: &JwtConfig) -> crate::Result<AccessToken> {
    let now = get_current_timestamp();
    let claims = AccessClaims {
        iat: now,
        exp: now + config.access_ttl.as_secs(),
        id,
    };
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.expose_secret().as_bytes()),
    )
    .context("create access token")
    .map_err(crate::Error::from)
}

pub fn create_refresh_token(
    secret: RefreshSecret,
    _config: &JwtConfig,
) -> crate::Result<RefreshToken> {
    // TODO jwt
    Ok(secret.0.to_string())
}

pub fn parse_access_token(token: &str, config: &JwtConfig) -> crate::Result<UserId> {
    jsonwebtoken::decode::<AccessClaims>(
        token,
        &DecodingKey::from_secret(config.secret.expose_secret().as_bytes()),
        &Validation::default(),
    )
    .map(|token| token.claims.id)
    .context("decode jwt token")
    .map_err(crate::Error::from)
}

impl fmt::Debug for RefreshSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RefreshSecret()")
    }
}
