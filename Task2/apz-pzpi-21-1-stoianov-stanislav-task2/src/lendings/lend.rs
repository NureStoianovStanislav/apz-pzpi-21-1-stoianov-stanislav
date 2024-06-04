use crate::{database::Database, state::AppState, telemetry, Error};

use super::{due_date::DueDate, lending_date::LendingDate, NewLending};

#[tracing::instrument(skip(state))]
pub async fn lend_book(
    lending: NewLending,
    state: AppState,
) -> crate::Result<()> {
    let lendee_id = lending
        .lendee_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::LoggedOff)
        .inspect_err(telemetry::debug)?;
    let book_id = lending
        .book_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    let lent_on = LendingDate::new(lending.lent_on)?;
    let due = DueDate::new(lent_on.clone(), lending.lent_for);
    let lending = DbLending {
        book_id,
        lendee_id,
        lent_on,
        due,
    };
    save_lending(&lending, &state.database).await
}

#[derive(Clone, Debug)]
struct DbLending {
    book_id: i64,
    lendee_id: i64,
    lent_on: LendingDate,
    due: DueDate,
}

#[tracing::instrument(skip(db), err(Debug))]
async fn save_lending(lending: &DbLending, db: &Database) -> crate::Result<()> {
    sqlx::query(
        "
        insert into lendings
          (book_id, lendee_id, lent_on, due)
        values
          ($1, $2, $3, $4);
        ",
    )
    .bind(lending.book_id)
    .bind(lending.lendee_id)
    .bind(&lending.lent_on)
    .bind(&lending.due)
    .execute(db)
    .await
    .map(|_| ())
    .map_err(Error::from)
}
