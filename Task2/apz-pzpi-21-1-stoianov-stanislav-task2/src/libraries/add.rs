use crate::{
    auth::{check_permission, Role, UserId},
    database::Database,
    state::AppState,
    telemetry, Error,
};

use super::{
    address::Address, currency::Currency, daily_rate::DailyRate, name::Name,
    overdue_rate::OverdueRate, NewLibrary,
};

#[tracing::instrument(skip(state))]
pub async fn add_library(
    admin_id: UserId,
    library: NewLibrary,
    state: AppState,
) -> crate::Result<()> {
    check_permission(admin_id, &state, |role| {
        matches!(role, Role::Administrator)
    })
    .await?;
    let owner_id = library
        .owner_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    let library = CreateLibrary {
        owner_id,
        name: Name::new(library.name)?,
        address: Address::new(library.address)?,
        daily_rate: DailyRate::new(library.daily_rate)?,
        overdue_rate: OverdueRate::new(library.overdue_rate)?,
        currency: Currency::new(library.currency)?,
    };
    create_library(&library, &state.database).await
}

#[derive(Clone, Debug)]
struct CreateLibrary {
    owner_id: i64,
    name: Name,
    address: Address,
    daily_rate: DailyRate,
    overdue_rate: OverdueRate,
    currency: Currency,
}

#[tracing::instrument(skip(db), err(Debug))]
async fn create_library(
    library: &CreateLibrary,
    db: &Database,
) -> crate::Result<()> {
    sqlx::query(
        "
        insert into libraries
          (name, address, daily_rate, overdue_rate, currency, owner_id)
        values
          ($1, $2, $3, $4, $5, $6);
        ",
    )
    .bind(&library.name)
    .bind(&library.address)
    .bind(&library.daily_rate)
    .bind(&library.overdue_rate)
    .bind(&library.currency)
    .bind(library.owner_id)
    .execute(db)
    .await
    .map(|_| ())
    .map_err(Error::from)
}
