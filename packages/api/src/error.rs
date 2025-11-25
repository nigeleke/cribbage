use dioxus::{
    fullstack::AsStatusCode,
    prelude::{ServerFnError, StatusCode},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum ApiError {
    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("not found")]
    NotFound,

    #[error("domain error: {0}")]
    Domain(String),

    #[cfg(not(debug_assertions))]
    #[error("unexpected")]
    Unexpected,

    #[cfg(debug_assertions)]
    #[error("unexpected")]
    Unexpected { message: String },
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
