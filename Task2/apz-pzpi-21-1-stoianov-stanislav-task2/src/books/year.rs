use serde::Serialize;

use crate::Error;

pub type UnvalidatedYear = i16;

#[derive(Clone, Debug, Default, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Year(UnvalidatedYear);

impl Year {
    pub fn new(year: UnvalidatedYear) -> crate::Result<Self> {
        if year > 2024 {
            Err(Error::Validation("year cannot be from the future"))
        } else {
            Ok(Self(year))
        }
    }
}

impl TryFrom<UnvalidatedYear> for Year {
    type Error = Error;

    fn try_from(value: UnvalidatedYear) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Year> for UnvalidatedYear {
    fn from(value: Year) -> Self {
        value.0
    }
}
