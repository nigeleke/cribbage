use dioxus::prelude::*;

#[component]
pub fn NotFoundPage(segments: Vec<String>) -> Element {
    rsx! {
        p {"Not Found page"}
        ul {
            {segments.iter().map(|s| rsx! { li { {s.to_string()} } })}
        }
    }
}
