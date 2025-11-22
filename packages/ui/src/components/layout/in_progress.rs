use dioxus::prelude::*;

#[component]
pub fn InProgress(
    north: Element,
    south: Element,
    east: Element,
    west: Element,
    centre: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/in_progress.css") }
        div {
            class: "in-progress",
            div {
                class: "in-progress__east",
                {east}
            }
            div {
                class: "in-progress__north",
                {north}
            }
            div {
                class: "in-progress__centre",
                {centre}
            }
            div {
                class: "in-progress__south",
                {south}
            }
            div {
                class: "in-progress__west",
                {west}
            }
        }
    }
}
