use anyhow::Context;
use jsonwebtoken::{get_current_timestamp, EncodingKey, Header};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::JwtConfig, database::DbSecret};

use super::UserId;

pub type RefreshToken = DbSecret<Uuid>;

pub type AccessToken = Secret<String>;

pub fn new_refresh_secret() -> RefreshToken {
    DbSecret::new(Uuid::new_v4())
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessClaims {
    iat: u64,
    exp: u64,
    id: UserId,
}

#[tracing::instrument(skip(config), err(Debug))]
pub fn access_token(id: UserId, config: &JwtConfig) -> crate::Result<AccessToken> {
    let now = get_current_timestamp();
    let claims = AccessClaims {
        iat: now,
        exp: now + config.access_ttl.as_secs(),
        id,
    };
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.expose_secret().as_bytes()),
    )
    .map(Secret::new)
    .context("create access token")?;
    Ok(token)
}
