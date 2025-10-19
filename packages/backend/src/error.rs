use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("{0}")]
    VarError(#[from] std::env::VarError),

    #[error("{0}")]
    DatabaseError(#[from] crate::database::DatabaseError),

    #[error("{0}")]
    SqlxError(#[from] sqlx::error::Error),

    #[error("{0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("{0}")]
    DomainError(#[from] crate::domain::DomainError),

    #[error("{0}")]
    StrumParseError(#[from] strum::ParseError),
}
