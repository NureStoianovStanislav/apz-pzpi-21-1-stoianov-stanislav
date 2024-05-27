mod sign_up;
mod token;

pub use sign_up::sign_up;

mod email;
mod password;

use serde::Deserialize;

use crate::id::{tag, Id};

use self::{
    email::{Email, UnvalidatedEmail},
    password::{PasswordHash, UnvalidatedPassword},
    token::RefreshToken,
};

pub type UserId = Id<{ tag("user") }>;

#[derive(Clone, Debug, Deserialize)]
pub struct Credentials {
    email: UnvalidatedEmail,
    password: UnvalidatedPassword,
}

#[derive(Clone, Debug, sqlx::Type, sqlx::FromRow)]
struct NewUser {
    email: Email,
    password_hash: PasswordHash,
    refresh_token: RefreshToken,
}
