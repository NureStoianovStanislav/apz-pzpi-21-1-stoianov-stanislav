mod author;
mod genre;
mod name;
mod year;

mod add;
mod view;

pub use add::add_book;
pub use view::{list_library_books, view_book};

use serde::{Deserialize, Serialize};

use crate::id::{tag, Id};

use self::{
    author::{Author, UnvalidatedAuthor},
    genre::{Genre, UnvalidatedGenre},
    name::{Name, UnvalidatedName},
    year::{UnvalidatedYear, Year},
};

pub type BookId = Id<{ tag("book") }>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBook {
    pub year: UnvalidatedYear,
    pub name: UnvalidatedName,
    pub genre: UnvalidatedGenre,
    pub author: UnvalidatedAuthor,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub id: BookId,
    pub year: Year,
    pub name: Name,
    pub genre: Genre,
    pub author: Author,
}
