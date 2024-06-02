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
