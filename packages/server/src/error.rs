use anyhow::Error as AnyhowError;
use thiserror::*;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("forbidden request: {0}")]
    Forbidden(String),

    #[error("not found")]
    NotFound,

    #[error(transparent)]
    Domain(#[from] crate::domain::DomainError),

    #[error("internal server error")]
    Internal(
        #[from]
        #[source]
        AnyhowError,
    ),
}

impl ServerError {
    pub fn bug(msg: impl std::fmt::Display) -> Self {
        ServerError::Internal(anyhow::anyhow!("BUG: {msg}"))
    }

    pub fn bug_fmt(msg: impl std::fmt::Display, args: std::fmt::Arguments<'_>) -> Self {
        ServerError::Internal(anyhow::anyhow!("BUG: {msg}{args}"))
    }
}
