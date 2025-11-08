use dioxus::prelude::*;

#[component]
pub fn ErrorPage(error: String) -> Element {
    rsx! {
        div {
           h2 { "Unexpected Error" }
           ul {
                li { {error.to_string()} }
           }
       }
    }
}
