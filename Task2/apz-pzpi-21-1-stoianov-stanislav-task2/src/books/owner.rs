use crate::{database::Database, telemetry, Error};

#[tracing::instrument(skip(db))]
pub async fn check_owns(
    owner_id: i64,
    library_id: i64,
    db: &Database,
) -> crate::Result<()> {
    sqlx::query_as(
        "
        select from libraries
        where id = $1
          and owner_id = $2;
        ",
    )
    .bind(library_id)
    .bind(owner_id)
    .fetch_optional(db)
    .await
    .map_err(Error::from)
    .and_then(|row| {
        row.ok_or(Error::Unauthorized).inspect_err(telemetry::debug)
    })
}
