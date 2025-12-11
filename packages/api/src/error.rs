use dioxus::{
    fullstack::AsStatusCode,
    prelude::{ServerFnError, StatusCode},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents errors that can occur in the API layer.
#[derive(Debug, Serialize, Deserialize, Error)]
pub enum ApiError {
    /// Indicates a client-side error, such as invalid input or malformed request.
    #[error("bad request: {0}")]
    BadRequest(String),

    /// Indicates that the user is not authorized to perform the requested action
    #[error("forbidden: {0}")]
    Forbidden(String),

    /// Indicates that the requested resource could not be found.
    #[error("not found")]
    NotFound,

    /// Represents a domain-level error, originating from business logic validation.
    #[error("domain error: {0}")]
    Domain(String),

    /// Represents an unexpected error in production builds.
    #[cfg(not(debug_assertions))]
    #[error("An unexpected error occurred")]
    Unexpected,

    /// Represents an unexpected error in debug builds, including a descriptive message.
    #[cfg(debug_assertions)]
    #[error("An unexpected error occurred {message}")]
    Unexpected {
        /// Description of what happened and / or where.
        message: String,
    },
}

impl AsStatusCode for ApiError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::FORBIDDEN,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Domain(_) => StatusCode::BAD_REQUEST,
            #[cfg(not(debug_assertions))]
            ApiError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
            #[cfg(debug_assertions)]
            ApiError::Unexpected { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ServerFnError> for ApiError {
    fn from(value: ServerFnError) -> Self {
        #[cfg(not(debug_assertions))]
        {
            ApiError::Unexpected
        }

        #[cfg(debug_assertions)]
        {
            ApiError::Unexpected {
                message: value.to_string(),
            }
        }
    }
}

#[cfg(feature = "server")]
mod server_only {
    use dioxus::prelude::*;
    use server::error::ServerError;

    use super::*;

    impl From<ServerError> for ApiError {
        fn from(err: ServerError) -> Self {
            match err {
                ServerError::Forbidden(msg) => ApiError::Forbidden(msg),
                ServerError::NotFound => ApiError::NotFound,
                ServerError::Domain(e) => ApiError::Domain(e.to_string()),
                ServerError::Internal(e) => {
                    error!("Internal server error: {e:#}");
                    #[cfg(debug_assertions)]
                    {
                        ApiError::Unexpected {
                            message: e.to_string(),
                        }
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        ApiError::Unexpected
                    }
                }
            }
        }
    }
}
