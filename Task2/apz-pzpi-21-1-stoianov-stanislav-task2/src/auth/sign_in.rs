use crate::{database::Database, state::AppState, telemetry};

use super::{
    email::UnvalidatedEmail,
    password::{verify_password, PasswordHash},
    token::{create_access_token, create_refresh_token, RefreshSecret},
    Credentials, TokenPair, UserId,
};

#[tracing::instrument(skip(state))]
pub async fn sign_in(
    credentials: Credentials,
    state: AppState,
) -> crate::Result<TokenPair> {
    let (user_data, hash) = get_user(&credentials.email, &state.database)
        .await?
        .map(|u| ((u.id, u.refresh_secret), u.password_hash))
        .unzip();
    telemetry::instrument_blocking(move || {
        verify_password(
            &credentials.password,
            hash.as_ref(),
            (*state.hasher_config).clone(),
        )
    })
    .await??;
    let (id, refresh_secret) = user_data.unwrap();
    let id = UserId::new(id, &state.id_cipher);
    let access_token = create_access_token(id, &state.jwt_config)?;
    let refresh_token =
        create_refresh_token(refresh_secret, &state.jwt_config)?;
    Ok(TokenPair {
        access_token,
        refresh_token,
    })
}

#[derive(Clone, Debug, sqlx::FromRow)]
struct DbUser {
    id: i64,
    password_hash: PasswordHash,
    refresh_secret: RefreshSecret,
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_user(
    email: &UnvalidatedEmail,
    db: &Database,
) -> crate::Result<Option<DbUser>> {
    sqlx::query_as(
        "
        select id, password_hash, refresh_secret
        from users
        where email = $1;
        ",
    )
    .bind(email)
    .fetch_optional(db)
    .await
    .map_err(crate::Error::from)
}
