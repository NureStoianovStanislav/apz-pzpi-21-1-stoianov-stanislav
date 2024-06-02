use core::fmt;

use crate::id;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Validation(&'static str),
    #[error("account already exists")]
    AccountExists,
    #[error("sign in to continue")]
    LoggedOff,
    #[error("wrong email or password")]
    InvalidCredentials,
    #[error("requested resource not found")]
    NotFound,
    #[error("no permission for the resourse")]
    Unauthorized,
    #[error("an unexpected error occurred")]
    Internal(#[from] ErrorChain),
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

impl From<id::DecodeError> for Error {
    fn from(value: id::DecodeError) -> Self {
        anyhow::Error::from(value).context("parse id").into()
    }
}

#[derive(thiserror::Error)]
#[error(transparent)]
pub struct ErrorChain(#[from] anyhow::Error);

impl fmt::Debug for ErrorChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")?;
        std::iter::successors(self.0.source(), |err| err.source())
            .try_for_each(|err| write!(f, ": {err}"))
    }
}
