use crate::components::DebouncedInput;
use dioxus::prelude::*;

#[component]
pub fn Filter(filter: String, on_filter_changed: Callback<String>) -> Element {
    let on_debounced_input = move |value: String| {
        spawn(async move {
            on_filter_changed.call(value);
        });
    };

    rsx! {
        DebouncedInput {
            value: filter,
            placeholder: "    Search games...",
            on_debounced_input,
        }
    }
}
