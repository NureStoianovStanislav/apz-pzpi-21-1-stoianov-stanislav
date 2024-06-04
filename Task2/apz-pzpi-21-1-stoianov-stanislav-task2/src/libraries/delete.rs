use crate::{
    auth::{check_permission, Role, UserId},
    database::Database,
    state::AppState,
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
    .await?;
    let id = library_id.sql_id(&state.id_cipher)?;
    delete_db_library(id, &state.database).await
}

#[tracing::instrument(skip(db), err(Debug))]
async fn delete_db_library(
    library_id: i64,
    db: &Database,
) -> crate::Result<()> {
    sqlx::query(
        "
        delete from libraries
        where id = $1;
        ",
    )
    .bind(library_id)
    .execute(db)
    .await
    .map(|_| ())
    .map_err(crate::Error::from)
}
