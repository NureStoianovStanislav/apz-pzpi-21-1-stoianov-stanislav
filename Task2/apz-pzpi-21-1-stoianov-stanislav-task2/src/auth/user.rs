use crate::{database::Database, state::AppState, Error};

use super::{email::Email, name::Name, UpdateUser, User, UserId};

#[derive(Clone, Debug, sqlx::FromRow)]
struct UserInfo {
    name: Name,
    email: Email,
}

#[derive(Clone, Debug)]
struct UpdateUserInfo {
    id: i64,
    name: Name,
}

#[tracing::instrument(skip(state))]
pub async fn get_user(user_id: UserId, state: AppState) -> crate::Result<User> {
    let db_id = user_id.sql_id(&state.id_cipher)?;
    let user_info = get_user_info(db_id, &state.database).await?;
    Ok(User {
        id: user_id,
        name: user_info.name,
        email: user_info.email,
    })
}

#[tracing::instrument(skip(state))]
pub async fn update_user(
    user_id: UserId,
    user_info: UpdateUser,
    state: AppState,
) -> crate::Result<()> {
    let user_info = UpdateUserInfo {
        id: user_id.sql_id(&state.id_cipher)?,
        name: Name::new(user_info.name)?,
    };
    update_user_info(&user_info, &state.database).await
}

#[tracing::instrument(skip(db))]
async fn get_user_info(user_id: i64, db: &Database) -> crate::Result<UserInfo> {
    sqlx::query_as(
        "
        select name, email 
        from users
        where id = $1;
        ",
    )
    .bind(user_id)
    .fetch_optional(db)
    .await
    .map_err(Error::from)
    .inspect_err(|e| tracing::error!("{e:?}"))
    .and_then(|user| {
        user.ok_or(Error::NotFound)
            .inspect_err(|e| tracing::debug!("{e:?}"))
    })
}

#[tracing::instrument(skip(db))]
async fn update_user_info(user_info: &UpdateUserInfo, db: &Database) -> crate::Result<()> {
    match sqlx::query(
        "
        update users
        set name = $1
        where id = $2;
        ",
    )
    .bind(&user_info.name)
    .bind(user_info.id)
    .execute(db)
    .await
    .inspect_err(|e| tracing::error!("{e:?}"))?
    .rows_affected()
    {
        0 => Err(Error::NotFound).inspect_err(|e| tracing::debug!("{e:?}")),
        1 => Ok(()),
        _ => unreachable!(),
    }
}
