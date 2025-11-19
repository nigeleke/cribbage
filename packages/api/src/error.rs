use thiserror::*;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("forbidden request: {0}")]
    Forbidden(String),

    #[error("not found")]
    NotFound,

    #[error("{0}")]
    Domain(String),

    #[error("an unexpected error occurred: {0}")]
    Unexpected(String),
}

#[cfg(feature = "server")]
mod server_only {
    use super::*;
    use server::error::ServerError;

    impl From<ServerError> for ApiError {
        fn from(value: ServerError) -> Self {
            match value {
                ServerError::Forbidden(error) => ApiError::Forbidden(error),
                ServerError::NotFound => ApiError::NotFound,
                ServerError::Domain(error) => ApiError::Domain(error.to_string()),
                ServerError::Internal(error) => ApiError::Unexpected(error.to_string()),
            }
        }
    }
}
