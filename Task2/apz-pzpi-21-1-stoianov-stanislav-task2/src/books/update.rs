use crate::{
    auth::UserId, database::Database, libraries::LibraryId, state::AppState,
    telemetry, Error,
};

use super::{
    author::Author, check_owns, genre::Genre, name::Name, year::Year, BookId,
    UpdateBook,
};

#[tracing::instrument(skip(state))]
pub async fn update_book(
    owner_id: UserId,
    library_id: LibraryId,
    book_id: BookId,
    book: UpdateBook,
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
    let book_id = book_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    let book = DbBook {
        id: book_id,
        year: Year::new(book.year)?,
        name: Name::new(book.name)?,
        genre: Genre::new(book.genre)?,
        author: Author::new(book.author)?,
    };
    update_db_book(book, &state.database).await
}

#[derive(Clone, Debug)]
struct DbBook {
    id: i64,
    year: Year,
    name: Name,
    genre: Genre,
    author: Author,
}

#[tracing::instrument(skip(db))]
async fn update_db_book(book: DbBook, db: &Database) -> crate::Result<()> {
    match sqlx::query(
        "
        update books
        set (year, name, genre, author)
          = ($1, $2, $3, $4)
        where id = $5;
        ",
    )
    .bind(&book.year)
    .bind(&book.name)
    .bind(&book.genre)
    .bind(&book.author)
    .bind(book.id)
    .execute(db)
    .await
    .map_err(Error::from)
    .inspect_err(telemetry::error)?
    .rows_affected()
    {
        0 => Err(Error::NotFound),
        1 => Ok(()),
        _ => unreachable!(),
    }
    .inspect_err(telemetry::debug)
}
