mod email;
mod name;
mod password;
mod role;
mod token;

mod sign_in;
mod sign_up;
mod user;

pub use role::Role;
pub use sign_in::sign_in;
pub use sign_up::sign_up;
pub use token::parse_access_token;
pub use user::{check_permission, get_all_users, get_user, update_user};

use serde::{Deserialize, Serialize};

use crate::id::{tag, Id};

use self::{
    email::{Email, UnvalidatedEmail},
    name::{Name, UnvalidatedName},
    password::UnvalidatedPassword,
    token::{AccessToken, RefreshToken},
};

pub type UserId = Id<{ tag("user") }>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub email: UnvalidatedEmail,
    pub password: UnvalidatedPassword,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    pub name: Name,
    pub email: Email,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub name: UnvalidatedName,
}

#[derive(Clone, Debug)]
pub struct TokenPair {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}
