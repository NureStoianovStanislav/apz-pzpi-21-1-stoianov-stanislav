use crate::{
    auth::{check_permission, Role, UserId},
    database::Database,
    state::AppState,
};

use super::{address::Address, name::Name, NewLibrary};

#[tracing::instrument(skip(state))]
pub async fn add_library(
    admin_id: UserId,
    library: NewLibrary,
    state: AppState,
) -> crate::Result<()> {
    check_permission(admin_id, &state, |role| matches!(role, Role::Administrator)).await?;
    let library = CreateLibrary {
        owner_id: library.owner_id.sql_id(&state.id_cipher)?,
        name: Name::new(library.name)?,
        address: Address::new(library.address)?,
    };
    create_library(&library, &state.database).await
}

#[derive(Clone, Debug)]
pub struct CreateLibrary {
    pub owner_id: i64,
    pub name: Name,
    pub address: Address,
}

async fn create_library(library: &CreateLibrary, db: &Database) -> crate::Result<()> {
    sqlx::query(
        "
        insert into libraries
          (name, address, owner_id)
        values
          ($1, $2, $3);
        ",
    )
    .bind(&library.name)
    .bind(&library.address)
    .bind(library.owner_id)
    .execute(db)
    .await
    .map(|_| ())
    .map_err(crate::Error::from)
}
