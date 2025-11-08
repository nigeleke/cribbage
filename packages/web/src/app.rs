use dioxus::prelude::*;
use ui::{Route, UnexpectedErrorPage};

/// The main cribbage application view.
#[component]
pub fn App() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |errors| {
                rsx! {
                    UnexpectedErrorPage { errors }
                }
            },
            Router::<Route> {}
        }
    }
}
