use serde::Serialize;
use sqlx::types::Decimal;

use crate::Error;

pub type UnvalidatedDailyRate = Decimal;

#[derive(Clone, Debug, Default, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct DailyRate(UnvalidatedDailyRate);

impl DailyRate {
    pub fn new(rate: UnvalidatedDailyRate) -> crate::Result<Self> {
        Ok(Self(rate))
    }
}

impl TryFrom<UnvalidatedDailyRate> for DailyRate {
    type Error = Error;

    fn try_from(value: UnvalidatedDailyRate) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<DailyRate> for UnvalidatedDailyRate {
    fn from(value: DailyRate) -> Self {
        value.0
    }
}
