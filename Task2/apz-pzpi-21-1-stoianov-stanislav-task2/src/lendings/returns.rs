use crate::{database::Database, state::AppState, telemetry, Error};

use super::{return_date::ReturnDate, ReturnRequest};

#[tracing::instrument(skip(state))]
pub async fn return_book(
    return_request: ReturnRequest,
    state: AppState,
) -> crate::Result<()> {
    let book_id = return_request
        .book_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    let today = ReturnDate::today();
    set_return_date(book_id, today, &state.database).await
}

#[tracing::instrument(skip(db), err(Debug))]
async fn set_return_date(
    book_id: i64,
    return_date: ReturnDate,
    db: &Database,
) -> crate::Result<()> {
    sqlx::query(
        "
        update lendings
        set returned_on = $1
        where book_id = $2;
        ",
    )
    .bind(&return_date)
    .bind(book_id)
    .execute(db)
    .await
    .map(|_| ())
    .map_err(Error::from)
}
