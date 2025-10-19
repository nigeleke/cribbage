use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("unexpected error: {0}")]
    BackendError(#[from] dioxus::prelude::ServerFnError),

    #[error("DELETE ME: {0}")]
    DeleteMe(String),
}
