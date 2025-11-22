use dioxus::prelude::*;

#[component]
pub fn LoadMore(has_more: ReadSignal<bool>, on_load_more: Callback<()>) -> Element {
    rsx! {
        button {
            class: "home-page__join-game__more-button",
            disabled: !has_more(),
            onclick: move |_| on_load_more(()),
            "More..."
        }
    }
}
