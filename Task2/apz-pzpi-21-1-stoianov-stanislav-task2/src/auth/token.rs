use core::fmt;

use anyhow::Context;
use jsonwebtoken::{
    get_current_timestamp, DecodingKey, EncodingKey, Header, Validation,
};
use secrecy::ExposeSecret;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::config::JwtConfig;

use super::UserId;

pub type AccessToken = String;

pub type RefreshToken = String;

#[derive(Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(transparent)]
#[serde(transparent)]
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

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshClaims {
    iat: u64,
    exp: u64,
    secret: RefreshSecret,
}

#[tracing::instrument(skip(config), err(Debug))]
pub fn create_access_token(
    id: UserId,
    config: &JwtConfig,
) -> crate::Result<AccessToken> {
    let now = get_current_timestamp();
    let claims = AccessClaims {
        iat: now,
        exp: now + config.access_ttl.as_secs(),
        id,
    };
    encode_claims(&claims, config)
}

pub fn create_refresh_token(
    secret: RefreshSecret,
    config: &JwtConfig,
) -> crate::Result<RefreshToken> {
    let now = get_current_timestamp();
    let claims = RefreshClaims {
        iat: now,
        exp: now + config.refresh_ttl.as_secs(),
        secret,
    };
    encode_claims(&claims, config)
}

pub fn parse_access_token(
    token: &str,
    config: &JwtConfig,
) -> crate::Result<UserId> {
    parse_claims::<AccessClaims>(token, config).map(|claims| claims.id)
}

#[allow(unused)]
pub fn parse_refresh_token(
    token: &str,
    config: &JwtConfig,
) -> crate::Result<RefreshSecret> {
    parse_claims::<RefreshClaims>(token, config).map(|claims| claims.secret)
}

fn encode_claims<C: Serialize>(
    claims: &C,
    config: &JwtConfig,
) -> crate::Result<String> {
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.key.expose_secret().as_bytes()),
    )
    .context("encode jwt")
    .map_err(crate::Error::from)
}

fn parse_claims<C: DeserializeOwned>(
    token: &str,
    config: &JwtConfig,
) -> crate::Result<C> {
    jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(config.key.expose_secret().as_bytes()),
        &Validation::default(),
    )
    .map(|token| token.claims)
    .context("decode jwt")
    .map_err(crate::Error::from)
}

impl fmt::Debug for RefreshSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RefreshSecret()")
    }
}
