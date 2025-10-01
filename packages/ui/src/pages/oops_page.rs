use dioxus::prelude::*;

#[component]
pub fn OopsPage(error: String) -> Element {
    rsx! {
        h2 { "Oops!!" }
        p { "Something went wrong." }
        p { {error} }
    }
}
