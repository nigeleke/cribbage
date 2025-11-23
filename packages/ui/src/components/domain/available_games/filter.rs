use crate::components::DebouncedInput;
use dioxus::prelude::*;

#[component]
pub fn Filter(on_filter_changed: Callback<String>) -> Element {
    let mut filter = use_signal(|| String::default());

    let on_debounced_input = move |value: String| {
        filter.set(value.clone());
        spawn(async move {
            on_filter_changed.call(value);
        });
    };

    rsx! {
        DebouncedInput {
            value: filter,
            placeholder: "🔍 Search games...",
            on_debounced_input,
        }
    }
}
