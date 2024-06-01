use crate::{database::Database, state::AppState, telemetry::instrument_blocking};

use super::{
    email::UnvalidatedEmail,
    password::{verify_password, PasswordHash},
    token::{access_token, RefreshToken},
    Credentials, TokenPair, UserId,
};

#[tracing::instrument(skip(state))]
pub async fn sign_in(credentials: Credentials, state: AppState) -> crate::Result<TokenPair> {
    let (user_data, hash) = get_user(&credentials.email, &state.database)
        .await?
        .map(|u| ((u.id, u.refresh_token), u.password_hash))
        .unzip();
    instrument_blocking(move || {
        verify_password(
            &credentials.password,
            hash.as_ref(),
            (*state.hasher_config).clone(),
        )
    })
    .await??;
    let (id, refresh_token) = user_data.unwrap();
    let id = UserId::new(id as u64, &state.id_cipher);
    let access_token = access_token(id, &state.jwt_config)?;
    Ok(TokenPair {
        access_token,
        refresh_token,
    })
}

#[derive(Clone, Debug, sqlx::FromRow)]
struct DbUser {
    id: i64,
    password_hash: PasswordHash,
    refresh_token: RefreshToken,
}

#[tracing::instrument(skip(db), err(Debug))]
async fn get_user(email: &UnvalidatedEmail, db: &Database) -> crate::Result<Option<DbUser>> {
    let hash = sqlx::query_as(
        "
        select id, password_hash, refresh_token
        from users
        where email = $1;
        ",
    )
    .bind(email)
    .fetch_optional(db)
    .await?;
    Ok(hash)
}
