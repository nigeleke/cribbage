use dioxus::fullstack::{FullstackContext, extract::FromRef};
use server::ServerState;

/// A wrapper around [`ServerState`] used for extracting the state
/// in contexts such as HTTP request handlers or middleware.
#[derive(Clone)]
pub struct ExtractedServerState(pub ServerState);

impl FromRef<FullstackContext> for ExtractedServerState {
    fn from_ref(context: &FullstackContext) -> Self {
        ExtractedServerState(
            context
                .extension::<ServerState>()
                .expect("ServerState Axum extension not set")
                .clone(),
        )
    }
}

/// Alias for [`ExtractedServerState`] used in HTTP request handlers
/// to extract the current server state.
pub use ExtractedServerState as ServerStateExtractor;
