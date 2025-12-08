use dioxus::fullstack::{FullstackContext, extract::FromRef};
use server::ServerState;

#[derive(Clone)]
pub struct ExtractedServerState(pub ServerState);

impl FromRef<FullstackContext> for ExtractedServerState {
    fn from_ref(ctx: &FullstackContext) -> Self {
        ExtractedServerState(
            ctx.extension::<ServerState>()
                .expect("ServerState Axum extension not set")
                .clone(),
        )
    }
}

pub use ExtractedServerState as ServerStateExtractor;
