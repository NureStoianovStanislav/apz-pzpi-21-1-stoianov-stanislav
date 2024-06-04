use serde::Serialize;

use crate::Error;

pub type UnvalidatedName = String;

#[derive(Clone, Debug, Default, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Name(UnvalidatedName);

impl Name {
    pub fn new(name: UnvalidatedName) -> crate::Result<Self> {
        if name.len() > 50 {
            Err(Error::Validation("name is too long"))
        } else {
            Ok(Self(name))
        }
    }
}

impl TryFrom<UnvalidatedName> for Name {
    type Error = Error;

    fn try_from(value: UnvalidatedName) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Name> for UnvalidatedName {
    fn from(value: Name) -> Self {
        value.0
    }
}
