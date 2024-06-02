use serde::Serialize;

use crate::Error;

pub type UnvalidatedAddress = String;

#[derive(Clone, Debug, Default, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Address(UnvalidatedAddress);

impl Address {
    pub fn new(address: UnvalidatedAddress) -> crate::Result<Self> {
        if address.len() > 100 {
            Err(Error::Validation("address is too long"))
        } else {
            Ok(Self(address))
        }
    }
}
