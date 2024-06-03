mod address;
mod name;

mod add;
mod view;

pub use add::add_library;
pub use view::{list_libraries, view_library};

use serde::{Deserialize, Serialize};

use crate::{
    auth::UserId,
    id::{tag, Id},
};

use self::{
    address::{Address, UnvalidatedAddress},
    name::{Name, UnvalidatedName},
};

pub type LibraryId = Id<{ tag("library") }>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewLibrary {
    pub owner_id: UserId,
    pub name: UnvalidatedName,
    pub address: UnvalidatedAddress,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub id: LibraryId,
    pub name: Name,
    pub address: Address,
}
