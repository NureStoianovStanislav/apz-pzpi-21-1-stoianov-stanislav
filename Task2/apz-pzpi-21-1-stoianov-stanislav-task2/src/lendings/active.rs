use crate::{
    auth::{get_user, UserId},
    books::{check_owns, view_book, BookId},
    database::Database,
    libraries::LibraryId,
    state::AppState,
    telemetry, Error,
};

use super::{due_date::DueDate, lending_date::LendingDate, Lending, LendingId};

#[tracing::instrument(skip(state))]
pub async fn active_lendings(
    owner_id: UserId,
    library_id: LibraryId,
    state: AppState,
) -> crate::Result<Vec<Lending>> {
    let db_owner_id = owner_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::LoggedOff)
        .inspect_err(telemetry::debug)?;
    let db_library_id = library_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    check_owns(db_owner_id, db_library_id, &state.database).await?;
    let db_lendings =
        get_active_lendings(db_library_id, &state.database).await?;
    let mut lendings = Vec::with_capacity(db_lendings.len());
    for lending in db_lendings {
        let id = LendingId::new(lending.id, &state.id_cipher);
        let book_id = BookId::new(lending.book_id, &state.id_cipher);
        let lendee_id = UserId::new(lending.lendee_id, &state.id_cipher);
        let book = view_book(library_id, book_id, state.clone()).await?;
        let lendee = get_user(lendee_id, state.clone()).await?;
        let lending = Lending {
            id,
            book,
            lendee,
            lent_on: lending.lent_on,
            due: lending.due,
        };
        lendings.push(lending);
    }
    Ok(lendings)
}

#[derive(Clone, Debug, sqlx::FromRow)]
struct DbLending {
    id: i64,
    book_id: i64,
    lendee_id: i64,
    lent_on: LendingDate,
    due: DueDate,
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_active_lendings(
    library_id: i64,
    db: &Database,
) -> crate::Result<Vec<DbLending>> {
    sqlx::query_as(
        "
        select id, book_id, lendee_id, lent_on, due
        from lendings
        where book_id in (
          select id from books where library_id = $1
        )
          and returned_on is null;
        ",
    )
    .bind(library_id)
    .fetch_all(db)
    .await
    .map_err(Error::from)
}
