use core::fmt;

use anyhow::Context;
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString,
    },
    Algorithm, Argon2, Params, Version,
};
use secrecy::{ExposeSecret, Secret};

use crate::{config::HasherConfig, error::Error};

pub type UnvalidatedPassword = Secret<String>;

#[derive(Clone, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct PasswordHash(String);

#[derive(Debug, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct Password(UnvalidatedPassword);

impl Password {
    pub fn new(value: UnvalidatedPassword) -> crate::Result<Self> {
        validate_password(value.expose_secret())
            .map(|_| Self(value))
            .map_err(Error::Validation)
    }
}

fn validate_password(password: &str) -> Result<(), &'static str> {
    match password {
        p if p.len() < 8 => Err("password must be at least 8 characters long"),
        p if !p.chars().any(char::is_lowercase) => {
            Err("password must contain at least one lowercase character")
        }
        p if !p.chars().any(char::is_uppercase) => {
            Err("password must contain at least one uppercase character")
        }
        p if !p.chars().any(char::is_numeric) => {
            Err("password must contain at least one number")
        }
        _ => Ok(()),
    }
}

#[tracing::instrument(skip_all, err(Debug))]
pub fn hash_password(
    password: &Password,
    config: HasherConfig,
) -> crate::Result<PasswordHash> {
    let hasher = hasher(config.key.expose_secret().as_bytes(), config.params)?;
    let password = password.0.expose_secret().as_bytes();
    hash_bytes(hasher, password)
}

#[tracing::instrument(skip_all, err(Debug, level = "debug"))]
pub fn verify_password(
    password: &UnvalidatedPassword,
    hash: Option<&PasswordHash>,
    config: HasherConfig,
) -> crate::Result<()> {
    let hasher = hasher(config.key.expose_secret().as_bytes(), config.params)?;
    let password = password.expose_secret().as_bytes();
    match hash {
        Some(hash) => {
            let hash = argon2::PasswordHash::new(&hash.0)
                .context("parse password hash")?;
            hasher
                .verify_password(password, &hash)
                .map_err(|_| Error::InvalidCredentials)
        }
        None => {
            hash_bytes(hasher, password).ok();
            Err(Error::InvalidCredentials)
        }
    }
}

fn hash_bytes(
    hasher: Argon2<'_>,
    password: &[u8],
) -> crate::Result<PasswordHash> {
    hasher
        .hash_password(password, &SaltString::generate(&mut OsRng))
        .map(|hash| PasswordHash(hash.to_string()))
        .context("hash password")
        .map_err(crate::Error::from)
}

fn hasher(secret: &[u8], params: Params) -> crate::Result<Argon2<'_>> {
    Argon2::new_with_secret(
        secret,
        Algorithm::default(),
        Version::default(),
        params,
    )
    .context("instantiate hasher")
    .map_err(crate::Error::from)
}

impl TryFrom<UnvalidatedPassword> for Password {
    type Error = Error;

    fn try_from(value: UnvalidatedPassword) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PasswordHash(...)")
    }
}
