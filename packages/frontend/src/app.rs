use dioxus::prelude::*;
use ui::{ErrorPage, Route};

/// The main cribbage application view.
#[component]
pub fn App() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |errors| {
                rsx! {
                    ErrorPage { errors }
                }
            },
            Router::<Route> {}
        }
    }
}
