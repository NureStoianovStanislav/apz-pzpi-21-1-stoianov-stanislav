mod address;
mod name;

mod add;

pub use add::add_library;

use serde::Deserialize;

use crate::auth::UserId;

use self::{address::UnvalidatedAddress, name::UnvalidatedName};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewLibrary {
    pub owner_id: UserId,
    pub name: UnvalidatedName,
    pub address: UnvalidatedAddress,
}
