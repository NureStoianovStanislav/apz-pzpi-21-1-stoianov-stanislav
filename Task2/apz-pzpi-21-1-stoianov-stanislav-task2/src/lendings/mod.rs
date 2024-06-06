mod due_date;
mod lending_date;
mod return_date;

mod active;
mod lend;
mod returns;

pub use active::active_lendings;
pub use lend::lend_book;
pub use returns::return_book;

use serde::{Deserialize, Serialize};

use crate::{
    auth::{User, UserId},
    books::{Book, BookId},
    id::{tag, Id},
};

pub use self::{
    due_date::{DueDate, LentFor},
    lending_date::{LendingDate, UnvalidatedLendingDate},
};

pub type LendingId = Id<{ tag("lending") }>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewLending {
    pub lendee_id: UserId,
    pub book_id: BookId,
    pub lent_on: UnvalidatedLendingDate,
    pub lent_for: LentFor,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Lending {
    pub id: LendingId,
    pub book: Book,
    pub lendee: User,
    pub lent_on: LendingDate,
    pub due: DueDate,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnRequest {
    pub book_id: BookId,
}
