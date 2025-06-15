use dioxus::prelude::*;

#[component]
pub fn NotFoundPage(segments: Vec<String>) -> Element {
    let path = segments.join("/");

    rsx! {
        p { "Page Not Found " {path} }
    }
}
