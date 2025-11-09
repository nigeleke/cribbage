use dioxus::prelude::*;

#[component]
pub fn ErrorPage(error: String) -> Element {
    rsx! {
        div {
           h2 { "An error occurred" }
           p { "{error}" }
       }
    }
}
