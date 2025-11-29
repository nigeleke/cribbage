use dioxus::prelude::*;

#[component]
pub fn ErrorPage(error: String) -> Element {
    error!("ErrorPage:error: '{error}'");
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/error_page.css") }
        div {
            class: "error-page",
            h2 { "Ooops!" }
            p { "Something went wrong."}
            p { "Please try again later."}
            p { "{error}" }
       }
    }
}
