use crate::{database::Database, state::AppState, telemetry, Error};

use super::{email::Email, name::Name, role::Role, UpdateUser, User, UserId};

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
pub async fn get_all_users(
    admin_id: UserId,
    state: AppState,
) -> crate::Result<Vec<User>> {
    check_permission(admin_id, &state, |role| {
        matches!(role, Role::Administrator)
    })
    .await
    .inspect_err(telemetry::debug)?;
    get_all_user_info(&state.database).await.map(|users| {
        users
            .into_iter()
            .map(|user_info| User {
                id: UserId::new(user_info.id, &state.id_cipher),
                name: user_info.name,
                email: user_info.email,
            })
            .collect()
    })
}

#[tracing::instrument(skip(state))]
pub async fn get_user(user_id: UserId, state: AppState) -> crate::Result<User> {
    let db_id = user_id
        .sql_id(&state.id_cipher)
        .map_err(|_| Error::NotFound)
        .inspect_err(telemetry::debug)?;
    get_user_info(db_id, &state.database)
        .await
        .and_then(|user| {
            user.ok_or(Error::NotFound).inspect_err(telemetry::debug)
        })
        .map(|user_info| User {
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

#[tracing::instrument(skip(state))]
pub async fn check_permission(
    user_id: UserId,
    state: &AppState,
    has_permissions: fn(Role) -> bool,
) -> crate::Result<()> {
    let db_id = user_id.sql_id(&state.id_cipher)?;
    get_user_role(db_id, &state.database)
        .await?
        .ok_or(Error::LoggedOff)
        .and_then(|role| {
            if has_permissions(role) {
                Ok(())
            } else {
                Err(Error::Unauthorized)
            }
        })
}

#[derive(Clone, Debug, sqlx::FromRow)]
struct AllUser {
    id: i64,
    name: Name,
    email: Email,
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_all_user_info(db: &Database) -> crate::Result<Vec<AllUser>> {
    sqlx::query_as(
        "
        select id, name, email 
        from users;
        ",
    )
    .fetch_all(db)
    .await
    .map_err(Error::from)
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_user_info(
    user_id: i64,
    db: &Database,
) -> crate::Result<Option<UserInfo>> {
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
}

#[tracing::instrument(skip(db))]
async fn update_user_info(
    user_info: &UpdateUserInfo,
    db: &Database,
) -> crate::Result<()> {
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
    .map_err(Error::from)
    .inspect_err(telemetry::error)?
    .rows_affected()
    {
        0 => Err(Error::NotFound).inspect_err(telemetry::debug),
        1 => Ok(()),
        _ => unreachable!(),
    }
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_user_role(
    user_id: i64,
    db: &Database,
) -> crate::Result<Option<Role>> {
    sqlx::query_as::<_, (_,)>(
        "
        select role
        from users
        where id = $1;
        ",
    )
    .bind(user_id)
    .fetch_optional(db)
    .await
    .map(|maybe| maybe.map(|role| role.0))
    .map_err(Error::from)
}
