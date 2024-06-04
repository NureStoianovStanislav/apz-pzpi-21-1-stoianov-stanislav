use crate::{
    auth::{check_permission, Role, UserId},
    database::Database,
    state::AppState,
    telemetry, Error,
};

use super::LibraryId;

#[tracing::instrument(skip(state))]
pub async fn delete_library(
    admin_id: UserId,
    library_id: LibraryId,
    state: AppState,
) -> crate::Result<()> {
    check_permission(admin_id, &state, |role| {
        matches!(role, Role::Administrator)
    })
    .await
    .map_err(|_| Error::LoggedOff)
    .inspect_err(telemetry::debug)?;
    let id = library_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    delete_db_library(id, &state.database).await
}

#[tracing::instrument(skip(db))]
async fn delete_db_library(
    library_id: i64,
    db: &Database,
) -> crate::Result<()> {
    match sqlx::query(
        "
        delete from libraries
        where id = $1;
        ",
    )
    .bind(library_id)
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
