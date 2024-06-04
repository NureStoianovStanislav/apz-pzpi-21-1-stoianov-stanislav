use serde::Serialize;

use crate::Error;

pub type UnvalidatedAuthor = String;

#[derive(Clone, Debug, Default, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Author(UnvalidatedAuthor);

impl Author {
    pub fn new(author: UnvalidatedAuthor) -> crate::Result<Self> {
        if author.len() > 50 {
            Err(Error::Validation("author name is too long"))
        } else {
            Ok(Self(author))
        }
    }
}

impl TryFrom<UnvalidatedAuthor> for Author {
    type Error = Error;

    fn try_from(value: UnvalidatedAuthor) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Author> for UnvalidatedAuthor {
    fn from(value: Author) -> Self {
        value.0
    }
}
