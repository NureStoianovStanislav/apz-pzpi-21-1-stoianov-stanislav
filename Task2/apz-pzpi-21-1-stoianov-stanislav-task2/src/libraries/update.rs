use crate::{
    auth::{check_permission, Role, UserId},
    database::Database,
    state::AppState,
    telemetry, Error,
};

use super::{
    address::Address, currency::Currency, daily_rate::DailyRate, name::Name,
    overdue_rate::OverdueRate, LibraryId, UpdateLibrary,
};

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
    .await
    .inspect_err(telemetry::debug)?;
    let library_id = library_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    let owner_id = library
        .owner_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    let library = DbLibrary {
        id: library_id,
        owner_id,
        name: Name::new(library.name)?,
        address: Address::new(library.address)?,
        daily_rate: DailyRate::new(library.daily_rate)?,
        overdue_rate: OverdueRate::new(library.overdue_rate)?,
        currency: Currency::new(library.currency)?,
    };
    update_db_library(&library, &state.database).await
}

#[derive(Clone, Debug)]
struct DbLibrary {
    id: i64,
    owner_id: i64,
    name: Name,
    address: Address,
    daily_rate: DailyRate,
    overdue_rate: OverdueRate,
    currency: Currency,
}

#[tracing::instrument(skip(db))]
async fn update_db_library(
    library: &DbLibrary,
    db: &Database,
) -> crate::Result<()> {
    match sqlx::query(
        "
        update libraries
        set (name, address, daily_rate, overdue_rate, currency, owner_id)
          = ($1, $2, $3, $4, $5, $6)
        where id = $7;
        ",
    )
    .bind(&library.name)
    .bind(&library.address)
    .bind(&library.daily_rate)
    .bind(&library.overdue_rate)
    .bind(&library.currency)
    .bind(library.owner_id)
    .bind(library.id)
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
