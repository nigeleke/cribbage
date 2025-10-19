use dioxus::prelude::*;

#[component]
pub fn ErrorPage(errors: ErrorContext) -> Element {
    rsx! {
        div {
           h2 { "Unexpected Error" }
           ul {
               if let Some(error) = errors.error() {
                   li { {error.to_string()} }
               }
           }
       }
    }
}
