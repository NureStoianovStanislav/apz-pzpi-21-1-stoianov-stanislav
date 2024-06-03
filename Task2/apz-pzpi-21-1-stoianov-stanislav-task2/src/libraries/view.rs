use crate::{database::Database, state::AppState, telemetry, Error};

use super::{address::Address, name::Name, Library, LibraryId};

#[tracing::instrument(skip(state))]
pub async fn list_libraries(state: AppState) -> crate::Result<Vec<Library>> {
    get_all_libraries(&state.database).await.map(|libraries| {
        libraries
            .into_iter()
            .map(|library| Library {
                id: LibraryId::new(library.id, &state.id_cipher),
                name: library.name,
                address: library.address,
            })
            .collect()
    })
}

#[tracing::instrument(skip(state))]
pub async fn view_library(
    id: LibraryId,
    state: AppState,
) -> crate::Result<Library> {
    let id = id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    get_library(id, &state.database)
        .await
        .and_then(|library| {
            library.ok_or(Error::NotFound).inspect_err(telemetry::debug)
        })
        .map(|library| Library {
            id: LibraryId::new(library.id, &state.id_cipher),
            name: library.name,
            address: library.address,
        })
}

#[derive(Clone, Debug, sqlx::FromRow)]
struct DbLibrary {
    id: i64,
    name: Name,
    address: Address,
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_all_libraries(db: &Database) -> crate::Result<Vec<DbLibrary>> {
    sqlx::query_as(
        "
        select id, name, address
        from libraries;
        ",
    )
    .fetch_all(db)
    .await
    .map_err(crate::Error::from)
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_library(
    id: i64,
    db: &Database,
) -> crate::Result<Option<DbLibrary>> {
    sqlx::query_as(
        "
        select id, name, address
        from libraries
        where id = $1;
        ",
    )
    .bind(id)
    .fetch_optional(db)
    .await
    .map_err(crate::Error::from)
}
