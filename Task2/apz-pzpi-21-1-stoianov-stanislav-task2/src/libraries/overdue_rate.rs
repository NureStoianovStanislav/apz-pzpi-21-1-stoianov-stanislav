use serde::Serialize;
use sqlx::types::Decimal;

use crate::Error;

pub type UnvalidatedOverdueRate = Decimal;

#[derive(Clone, Debug, Default, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct OverdueRate(UnvalidatedOverdueRate);

impl OverdueRate {
    pub fn new(rate: UnvalidatedOverdueRate) -> crate::Result<Self> {
        Ok(Self(rate))
    }
}

impl TryFrom<UnvalidatedOverdueRate> for OverdueRate {
    type Error = Error;

    fn try_from(value: UnvalidatedOverdueRate) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<OverdueRate> for UnvalidatedOverdueRate {
    fn from(value: OverdueRate) -> Self {
        value.0
    }
}
