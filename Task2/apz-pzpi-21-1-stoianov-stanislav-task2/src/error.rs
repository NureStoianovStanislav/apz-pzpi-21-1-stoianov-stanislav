use core::fmt;

use axum::{http::StatusCode, response::IntoResponse, Json};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("account already exists")]
    AccountExists,
    #[error("{0}")]
    Validation(&'static str),
    #[allow(private_interfaces)]
    #[error("an unexpected error occurred")]
    Internal(#[from] ErrorChain),
}

#[derive(serde::Serialize)]
struct ErrorMessage {
    error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            Error::AccountExists => StatusCode::CONFLICT,
            Error::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = ErrorMessage {
            error: self.to_string(),
        };
        (code, Json(message)).into_response()
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        ErrorChain::from(error).into()
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        anyhow::Error::from(error).context("execute sql").into()
    }
}

#[derive(thiserror::Error)]
#[error(transparent)]
struct ErrorChain(#[from] anyhow::Error);

impl fmt::Debug for ErrorChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")?;
        std::iter::successors(self.0.source(), |err| err.source())
            .try_for_each(|err| write!(f, "{err}"))
    }
}
