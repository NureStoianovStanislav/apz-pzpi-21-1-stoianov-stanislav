use crate::{
    auth::UserId, database::Database, libraries::LibraryId, state::AppState, telemetry, Error
};

use super::{author::Author, genre::Genre, name::Name, year::Year, NewBook};

#[tracing::instrument(skip(state))]
pub async fn add_book(
    owner_id: UserId,
    library_id: LibraryId,
    book: NewBook,
    state: AppState,
) -> crate::Result<()> {
    let book = DbBook {
        owner_id: owner_id.sql_id(&state.id_cipher)?,
        library_id: library_id.sql_id(&state.id_cipher)?,
        year: Year::new(book.year)?,
        name: Name::new(book.name)?,
        genre: Genre::new(book.genre)?,
        author: Author::new(book.author)?,
    };
    check_owns(book.owner_id, book.library_id, &state.database).await?;
    insert_book(book, &state.database).await
}

#[derive(Clone, Debug)]
struct DbBook {
    owner_id: i64,
    library_id: i64,
    year: Year,
    name: Name,
    genre: Genre,
    author: Author,
}

#[tracing::instrument(skip(db))]
async fn check_owns(
    owner_id: i64,
    library_id: i64,
    db: &Database,
) -> crate::Result<()> {
    sqlx::query_as(
        "
        select from libraries
        where id = $1
          and owner_id = $2;
        ",
    )
    .bind(library_id)
    .bind(owner_id)
    .fetch_optional(db)
    .await
    .map_err(Error::from)
    .and_then(|row| {
        row.ok_or(Error::Unauthorized).inspect_err(telemetry::debug)
    })
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
