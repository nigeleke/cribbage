use dioxus::prelude::*;

#[component]
pub fn ErrorPage(error: String) -> Element {
    debug!("ErrorPage:error: '{error}'");
    rsx! {
        div {
           h2 { "An error occurred" }
           p { "{error}" }
       }
    }
}
