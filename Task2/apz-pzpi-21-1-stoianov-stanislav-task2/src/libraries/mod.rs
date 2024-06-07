mod address;
mod rating;
mod currency;
mod daily_rate;
mod name;
mod overdue_rate;

mod add;
mod delete;
mod update;
mod view;

pub use add::add_library;
pub use delete::delete_library;
pub use update::update_library;
pub use view::{list_libraries, list_my_libraries, view_library};

use serde::{Deserialize, Serialize};

use crate::{
    auth::UserId,
    id::{tag, Id},
};

use self::{
    address::{Address, UnvalidatedAddress},
    currency::{Currency, UnvalidatedCurrency},
    daily_rate::{DailyRate, UnvalidatedDailyRate},
    name::{Name, UnvalidatedName},
    overdue_rate::{OverdueRate, UnvalidatedOverdueRate}, rating::Rating,
};

pub type LibraryId = Id<{ tag("library") }>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewLibrary {
    pub owner_id: UserId,
    pub name: UnvalidatedName,
    pub address: UnvalidatedAddress,
    pub daily_rate: UnvalidatedDailyRate,
    pub overdue_rate: UnvalidatedOverdueRate,
    pub currency: UnvalidatedCurrency,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub id: LibraryId,
    pub name: Name,
    pub address: Address,
    pub daily_rate: DailyRate,
    pub overdue_rate: OverdueRate,
    pub currency: Currency,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatedLibrary {
    pub id: LibraryId,
    pub name: Name,
    pub address: Address,
    pub daily_rate: DailyRate,
    pub overdue_rate: OverdueRate,
    pub currency: Currency,
    pub owner_id: UserId,
    pub rating: Rating
}


#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLibrary {
    pub owner_id: UserId,
    pub name: UnvalidatedName,
    pub address: UnvalidatedAddress,
    pub daily_rate: UnvalidatedDailyRate,
    pub overdue_rate: UnvalidatedOverdueRate,
    pub currency: UnvalidatedCurrency,
}
