mod email;
mod password;
mod sign_in;
mod sign_up;
mod token;

pub use sign_in::sign_in;
pub use sign_up::sign_up;

use serde::Deserialize;

use crate::id::{tag, Id};

use self::{
    email::UnvalidatedEmail,
    password::UnvalidatedPassword,
    token::{AccessToken, RefreshToken},
};

pub type UserId = Id<{ tag("user") }>;

#[derive(Clone, Debug, Deserialize)]
pub struct Credentials {
    pub email: UnvalidatedEmail,
    pub password: UnvalidatedPassword,
}

#[derive(Clone, Debug)]
pub struct TokenPair {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}
