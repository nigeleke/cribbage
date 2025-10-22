use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("unexpected error: {0}")]
    BackendError(#[from] dioxus::prelude::ServerFnError),
}
