use sqlx::error::ErrorKind;

use crate::{
    database::{error_kind, Database},
    error::Error,
    state::AppState,
    telemetry,
};

use super::{
    email::Email,
    name::Name,
    password::{hash_password, Password, PasswordHash},
    role::Role,
    token::RefreshSecret,
    Credentials,
};

#[derive(Clone, Debug, sqlx::Type, sqlx::FromRow)]
struct NewUser {
    name: Name,
    email: Email,
    password_hash: PasswordHash,
    refresh_secret: RefreshSecret,
    role: Role,
}

#[tracing::instrument(skip(state))]
pub async fn sign_up(
    credentials: Credentials,
    state: AppState,
) -> crate::Result<()> {
    let email = Email::new(credentials.email)?;
    let password = Password::new(credentials.password)?;
    let password_hash = telemetry::instrument_blocking(move || {
        hash_password(&password, (*state.hasher_config).clone())
    })
    .await??;
    let refresh_secret = RefreshSecret::new();
    let user = NewUser {
        name: Name::default(),
        email,
        password_hash,
        refresh_secret,
        role: Role::Client,
    };
    save_user(&user, &state.database).await
}

#[tracing::instrument(skip(db))]
async fn save_user(user: &NewUser, db: &Database) -> crate::Result<()> {
    match sqlx::query(
        "
        insert into users
          (name, email, password_hash, refresh_secret, role)
        values
          ($1, $2, $3, $4, $5);
        ",
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.refresh_secret)
    .bind(user.role)
    .execute(db)
    .await
    {
        Err(e) if error_kind(&e) == Some(ErrorKind::UniqueViolation) => {
            Err(Error::AccountExists).inspect_err(telemetry::debug)
        }
        other => other
            .map(|_| ())
            .map_err(Error::from)
            .inspect_err(telemetry::error),
    }
}
