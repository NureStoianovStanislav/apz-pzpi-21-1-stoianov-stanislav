use serde::Serialize;

use crate::Error;

pub type UnvalidatedGenre = String;

#[derive(Clone, Debug, Default, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Genre(UnvalidatedGenre);

impl Genre {
    pub fn new(genre: UnvalidatedGenre) -> crate::Result<Self> {
        if genre.len() > 50 {
            Err(Error::Validation("genre is too long"))
        } else {
            Ok(Self(genre))
        }
    }
}

impl TryFrom<UnvalidatedGenre> for Genre {
    type Error = Error;

    fn try_from(value: UnvalidatedGenre) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Genre> for UnvalidatedGenre {
    fn from(value: Genre) -> Self {
        value.0
    }
}
