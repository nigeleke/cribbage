use dioxus::prelude::*;

/// Display server errors to the user in a friendly way.
#[component]
pub fn Error(error: ReadSignal<Option<String>>) -> Element {
    rsx! {
        if let Some(error) = &*error.read() {
            div {
                class: "error",
                h4 { "Oops: An error occurred" }
                p { "This may be temporary, please try later. If the error persists please consider reporting:"}
                p { {error.to_string()} }
            }
        }
    }
}
