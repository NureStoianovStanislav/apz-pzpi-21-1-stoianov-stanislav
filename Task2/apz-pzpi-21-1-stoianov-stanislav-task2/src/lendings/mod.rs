mod due_date;
mod lending_date;

mod lend;

pub use lend::lend_book;

use serde::{Deserialize, Serialize};

use crate::{
    auth::UserId,
    books::BookId,
    id::{tag, Id},
};

use self::{
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
    pub book_id: BookId,
    pub lendee_id: UserId,
    pub lent_on: LendingDate,
    pub due: DueDate,
}
