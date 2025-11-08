use dioxus::prelude::*;

#[component]
pub fn UnexpectedErrorPage(errors: ErrorContext) -> Element {
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
