use crate::{
    auth::UserId,
    database::Database,
    lendings::{DueDate, LendingDate},
    state::AppState,
    telemetry, Error,
};

use super::{
    address::Address, currency::Currency, daily_rate::DailyRate, name::Name,
    overdue_rate::OverdueRate, Library, LibraryId, RatedLibrary,
};

#[tracing::instrument(skip(state))]
pub async fn list_libraries(state: AppState) -> crate::Result<Vec<Library>> {
    get_all_libraries(&state.database).await.map(|libraries| {
        libraries
            .into_iter()
            .map(|library| Library {
                id: LibraryId::new(library.id, &state.id_cipher),
                name: library.name,
                address: library.address,
                daily_rate: library.daily_rate,
                overdue_rate: library.overdue_rate,
                currency: library.currency,
            })
            .collect()
    })
}

#[tracing::instrument(skip(state))]
pub async fn list_my_libraries(
    user_id: UserId,
    state: AppState,
) -> crate::Result<Vec<Library>> {
    let user_id = user_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::LoggedOff)
        .inspect_err(telemetry::debug)?;
    get_user_libraries(user_id, &state.database)
        .await
        .map(|libraries| {
            libraries
                .into_iter()
                .map(|library| Library {
                    id: LibraryId::new(library.id, &state.id_cipher),
                    name: library.name,
                    address: library.address,
                    daily_rate: library.daily_rate,
                    overdue_rate: library.overdue_rate,
                    currency: library.currency,
                })
                .collect()
        })
}

#[tracing::instrument(skip(state))]
pub async fn view_library(
    id: LibraryId,
    state: AppState,
) -> crate::Result<RatedLibrary> {
    let id = id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    let library =
        get_library(id, &state.database).await.and_then(|library| {
            library.ok_or(Error::NotFound).inspect_err(telemetry::debug)
        })?;
    let owner_id =
        UserId::new(get_owner(id, &state.database).await?, &state.id_cipher);
    let rating =
        get_library_activity(id, &state.database)
            .await
            .map(|activity| {
                let num_lendings = activity.len() as i64;
                let business_days = activity.iter().sum::<i64>();
                business_days.checked_div(num_lendings).unwrap_or_default()
                    + num_lendings
            })?;
    Ok(RatedLibrary {
        id: LibraryId::new(library.id, &state.id_cipher),
        name: library.name,
        address: library.address,
        daily_rate: library.daily_rate,
        overdue_rate: library.overdue_rate,
        currency: library.currency,
        owner_id,
        rating,
    })
}

#[derive(Clone, Debug, sqlx::FromRow)]
struct DbLibrary {
    id: i64,
    name: Name,
    address: Address,
    daily_rate: DailyRate,
    overdue_rate: OverdueRate,
    currency: Currency,
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_owner(library_id: i64, db: &Database) -> crate::Result<i64> {
    sqlx::query_as::<_, (_,)>(
        "
        select owner_id
        from libraries
        where id = $1;
        ",
    )
    .bind(library_id)
    .fetch_one(db)
    .await
    .map(|r| r.0)
    .map_err(crate::Error::from)
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_all_libraries(db: &Database) -> crate::Result<Vec<DbLibrary>> {
    sqlx::query_as(
        "
        select id, name, address, daily_rate, overdue_rate, currency
        from libraries;
        ",
    )
    .fetch_all(db)
    .await
    .map_err(crate::Error::from)
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_user_libraries(
    user_id: i64,
    db: &Database,
) -> crate::Result<Vec<DbLibrary>> {
    sqlx::query_as(
        "
        select id, name, address, daily_rate, overdue_rate, currency
        from libraries
        where owner_id = $1;
        ",
    )
    .bind(user_id)
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
        select id, name, address, daily_rate, overdue_rate, currency
        from libraries
        where id = $1;
        ",
    )
    .bind(id)
    .fetch_optional(db)
    .await
    .map_err(crate::Error::from)
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_library_activity(
    library_id: i64,
    db: &Database,
) -> crate::Result<Vec<i64>> {
    sqlx::query_as::<_, (LendingDate, DueDate)>(
        "
        select lent_on, due 
        from lendings
        where book_id in (
          select id from books where library_id = $1
        );
        ",
    )
    .bind(library_id)
    .fetch_all(db)
    .await
    .map(|records| {
        records
            .into_iter()
            .map(|(lent_on, due)| due.lent_for(lent_on))
            .collect()
    })
    .map_err(crate::Error::from)
}
