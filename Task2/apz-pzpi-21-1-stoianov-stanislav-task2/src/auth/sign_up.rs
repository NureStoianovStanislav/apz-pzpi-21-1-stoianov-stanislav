use crate::{database::Database, state::AppState};

use super::{email::Email, password::Password, Credentials, NewUser, RefreshToken};

#[tracing::instrument(skip(state))]
pub async fn sign_up(credentials: Credentials, state: AppState) -> crate::Result<()> {
    let email = Email::new(credentials.email)?;
    let password_hash = Password::new(credentials.password).and_then(|p| p.hash(state.hasher))?;
    let refresh_token = RefreshToken::generate();
    let user = NewUser {
        email,
        password_hash,
        refresh_token,
    };
    save_user(&user, &state.database).await
}

#[tracing::instrument(skip(db), err(Debug))]
async fn save_user(user: &NewUser, db: &Database) -> crate::Result<()> {
    sqlx::query(
        "
        insert into users
          (email, password_hash, refresh_token)
        values
          ($1, $2, $3);
        ",
    )
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.refresh_token)
    .execute(db)
    .await?;
    Ok(())
}
