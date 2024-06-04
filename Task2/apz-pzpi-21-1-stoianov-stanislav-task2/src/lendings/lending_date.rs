use chrono::{Local, NaiveDate};
use serde::Serialize;

use crate::Error;

pub type UnvalidatedLendingDate = NaiveDate;

#[derive(Clone, Debug, Default, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct LendingDate(UnvalidatedLendingDate);

impl LendingDate {
    pub fn new(date: UnvalidatedLendingDate) -> crate::Result<Self> {
        if date > Local::now().date_naive() {
            Err(Error::Validation("cannot lend books from the future"))
        } else {
            Ok(Self(date))
        }
    }
}

impl TryFrom<UnvalidatedLendingDate> for LendingDate {
    type Error = Error;

    fn try_from(value: UnvalidatedLendingDate) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<LendingDate> for UnvalidatedLendingDate {
    fn from(value: LendingDate) -> Self {
        value.0
    }
}
