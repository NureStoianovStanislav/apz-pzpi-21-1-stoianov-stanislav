use secrecy::ExposeSecret;
use sqlx::{
    error::{DatabaseError, ErrorKind},
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    Pool, Postgres,
};

use crate::config::DatabaseConfig;

pub type Database = Pool<Postgres>;

pub fn connect(config: DatabaseConfig) -> Database {
    let connect_options = PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.user)
        .password(config.password.expose_secret())
        .database(&config.database)
        .ssl_mode(if config.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        });
    PgPoolOptions::new().connect_lazy_with(connect_options)
}

pub fn error_kind(error: &sqlx::Error) -> Option<ErrorKind> {
    error.as_database_error().map(DatabaseError::kind)
}
