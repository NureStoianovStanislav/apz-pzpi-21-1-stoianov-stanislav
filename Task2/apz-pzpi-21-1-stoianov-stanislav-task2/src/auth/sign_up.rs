use sqlx::error::ErrorKind;

use crate::{
    database::{error_kind, Database},
    error::Error,
    state::AppState,
    telemetry::instrument_blocking,
};

use super::{
    email::Email,
    password::{hash_password, Password, PasswordHash},
    token::RefreshSecret,
    Credentials,
};

#[derive(Clone, Debug, sqlx::Type, sqlx::FromRow)]
struct NewUser {
    email: Email,
    password_hash: PasswordHash,
    refresh_secret: RefreshSecret,
}

#[tracing::instrument(skip(state))]
pub async fn sign_up(credentials: Credentials, state: AppState) -> crate::Result<()> {
    let email = Email::new(credentials.email)?;
    let password = Password::new(credentials.password)?;
    let password_hash =
        instrument_blocking(move || hash_password(&password, (*state.hasher_config).clone()))
            .await??;
    let refresh_secret = RefreshSecret::new();
    let user = NewUser {
        email,
        password_hash,
        refresh_secret,
    };
    save_user(&user, &state.database).await
}

#[tracing::instrument(skip(db))]
async fn save_user(user: &NewUser, db: &Database) -> crate::Result<()> {
    match sqlx::query(
        "
        insert into users
          (email, password_hash, refresh_secret)
        values
          ($1, $2, $3);
        ",
    )
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.refresh_secret)
    .execute(db)
    .await
    {
        Err(e) if error_kind(&e) == Some(ErrorKind::UniqueViolation) => {
            Err(Error::AccountExists).inspect_err(|e| tracing::debug!("{e:?}"))
        }
        other => other
            .map(|_| ())
            .map_err(Error::from)
            .inspect_err(|e| tracing::error!("{e:?}")),
    }
}
