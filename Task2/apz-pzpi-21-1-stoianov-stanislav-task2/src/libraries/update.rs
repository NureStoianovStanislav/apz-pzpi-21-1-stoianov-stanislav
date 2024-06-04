use crate::{
    auth::{check_permission, Role, UserId},
    database::Database,
    state::AppState,
};

use super::{address::Address, name::Name, LibraryId, UpdateLibrary};

#[tracing::instrument(skip(state))]
pub async fn update_library(
    admin_id: UserId,
    library_id: LibraryId,
    library: UpdateLibrary,
    state: AppState,
) -> crate::Result<()> {
    check_permission(admin_id, &state, |role| {
        matches!(role, Role::Administrator)
    })
    .await?;
    let library = DbLibrary {
        id: library_id.sql_id(&state.id_cipher)?,
        owner_id: library.owner_id.sql_id(&state.id_cipher)?,
        name: Name::new(library.name)?,
        address: Address::new(library.address)?,
    };
    update_db_library(&library, &state.database).await
}

#[derive(Clone, Debug)]
struct DbLibrary {
    id: i64,
    owner_id: i64,
    name: Name,
    address: Address,
}

#[tracing::instrument(skip(db), err(Debug))]
async fn update_db_library(
    library: &DbLibrary,
    db: &Database,
) -> crate::Result<()> {
    sqlx::query(
        "
        update libraries
        set (name, address, owner_id)
          = ($1, $2, $3)
        where id = $4;
        ",
    )
    .bind(&library.name)
    .bind(&library.address)
    .bind(library.owner_id)
    .bind(library.id)
    .execute(db)
    .await
    .map(|_| ())
    .map_err(crate::Error::from)
}
