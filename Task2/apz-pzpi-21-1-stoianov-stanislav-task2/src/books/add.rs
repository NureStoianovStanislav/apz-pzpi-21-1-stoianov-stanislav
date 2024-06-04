use crate::{
    auth::UserId, database::Database, libraries::LibraryId, state::AppState,
    telemetry, Error,
};

use super::{
    author::Author, check_owns, genre::Genre, name::Name, year::Year, NewBook,
};

#[tracing::instrument(skip(state))]
pub async fn add_book(
    owner_id: UserId,
    library_id: LibraryId,
    book: NewBook,
    state: AppState,
) -> crate::Result<()> {
    let owner_id = owner_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::LoggedOff)
        .inspect_err(telemetry::debug)?;
    let library_id = library_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    check_owns(owner_id, library_id, &state.database).await?;
    let book = DbBook {
        library_id,
        year: Year::new(book.year)?,
        name: Name::new(book.name)?,
        genre: Genre::new(book.genre)?,
        author: Author::new(book.author)?,
    };
    insert_book(book, &state.database).await
}

#[derive(Clone, Debug)]
struct DbBook {
    library_id: i64,
    year: Year,
    name: Name,
    genre: Genre,
    author: Author,
}

#[tracing::instrument(skip(db), err(Debug))]
async fn insert_book(book: DbBook, db: &Database) -> crate::Result<()> {
    sqlx::query(
        "
        insert into books
          (year, name, genre, author, library_id)
        values
          ($1, $2, $3, $4, $5);
        ",
    )
    .bind(&book.year)
    .bind(&book.name)
    .bind(&book.genre)
    .bind(&book.author)
    .bind(book.library_id)
    .execute(db)
    .await
    .map(|_| ())
    .map_err(Error::from)
}
