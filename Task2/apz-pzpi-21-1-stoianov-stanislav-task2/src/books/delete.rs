use crate::{
    auth::UserId, database::Database, libraries::LibraryId, state::AppState,
    telemetry, Error,
};

use super::{check_owns, BookId};

#[tracing::instrument(skip(state))]
pub async fn delete_book(
    owner_id: UserId,
    library_id: LibraryId,
    book_id: BookId,
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
    delete_db_book(book_id, &state.database).await
}

#[tracing::instrument(skip(db))]
async fn delete_db_book(book_id: i64, db: &Database) -> crate::Result<()> {
    match sqlx::query(
        "
        delete from books
        where id = $1;
        ",
    )
    .bind(book_id)
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
