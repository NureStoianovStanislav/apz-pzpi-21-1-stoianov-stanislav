use serde::Serialize;

use crate::Error;

pub type UnvalidatedCurrency = String;

#[derive(Clone, Debug, Default, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Currency(UnvalidatedCurrency);

impl Currency {
    pub fn new(currency: UnvalidatedCurrency) -> crate::Result<Self> {
        match currency.as_str() {
            "UAH" | "USD" | "EUR" => Ok(Self(currency)),
            _ => Err(Error::Validation("unsupported currency")),
        }
    }
}

impl TryFrom<UnvalidatedCurrency> for Currency {
    type Error = Error;

    fn try_from(value: UnvalidatedCurrency) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Currency> for UnvalidatedCurrency {
    fn from(value: Currency) -> Self {
        value.0
    }
}
