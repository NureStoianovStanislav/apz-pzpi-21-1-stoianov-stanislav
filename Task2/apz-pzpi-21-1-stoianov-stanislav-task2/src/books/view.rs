use crate::{
    database::Database, libraries::LibraryId, state::AppState, telemetry, Error,
};

use super::{
    author::Author, genre::Genre, name::Name, year::Year, Book, BookId,
};

#[tracing::instrument(skip(state))]
pub async fn list_library_books(
    library_id: LibraryId,
    state: AppState,
) -> crate::Result<Vec<Book>> {
    let library_id = library_id.sql_id(&state.id_cipher)?;
    get_library_books(library_id, &state.database)
        .await
        .map(|books| {
            books
                .into_iter()
                .map(|book| Book {
                    id: BookId::new(book.id, &state.id_cipher),
                    year: book.year,
                    name: book.name,
                    genre: book.genre,
                    author: book.author,
                })
                .collect()
        })
}

#[tracing::instrument(skip(state))]
pub async fn view_book(
    library_id: LibraryId,
    book_id: BookId,
    state: AppState,
) -> crate::Result<Book> {
    let library_id = library_id.sql_id(&state.id_cipher)?;
    let book_id = book_id.sql_id(&state.id_cipher)?;
    get_book(library_id, book_id, &state.database)
        .await
        .and_then(|book| {
            book.ok_or(Error::NotFound).inspect_err(telemetry::debug)
        })
        .map(|book| Book {
            id: BookId::new(book.id, &state.id_cipher),
            year: book.year,
            name: book.name,
            genre: book.genre,
            author: book.author,
        })
}

#[derive(Clone, Debug, sqlx::FromRow)]
struct DbBook {
    id: i64,
    year: Year,
    name: Name,
    genre: Genre,
    author: Author,
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_library_books(
    library_id: i64,
    db: &Database,
) -> crate::Result<Vec<DbBook>> {
    sqlx::query_as(
        "
        select id, year, name, genre, author
        from books
        where library_id = $1;
        ",
    )
    .bind(library_id)
    .fetch_all(db)
    .await
    .map_err(Error::from)
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_book(
    library_id: i64,
    book_id: i64,
    db: &Database,
) -> crate::Result<Option<DbBook>> {
    sqlx::query_as(
        "
        select id, year, name, genre, author
        from books
        where id = $1
          and library_id = $2;
        ",
    )
    .bind(book_id)
    .bind(library_id)
    .fetch_optional(db)
    .await
    .map_err(Error::from)
}
