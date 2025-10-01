use dioxus::prelude::*;

#[component]
pub fn ErrorPage(errors: ErrorContext) -> Element {
    rsx! {
        div {
           h2 { "Unexpected Error" }
           ul {
               for error in errors.errors() {
                   li { {error.to_string()} }
               }
           }
       }
    }
}
