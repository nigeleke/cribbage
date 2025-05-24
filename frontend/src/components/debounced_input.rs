use std::time::Duration;

use dioxus::prelude::*;
use dioxus_sdk::utils::timing::use_debounce;

#[component]
pub fn DebouncedInput(
    value: ReadOnlySignal<String>,
    on_debounced_input: EventHandler<String>,
    placeholder: Option<String>,
) -> Element {
    let mut value = use_signal(|| value());

    let mut debounce = use_debounce(Duration::from_millis(300), move |value| {
        on_debounced_input.call(value);
    });

    rsx! {
        input {
            placeholder,
            value,
            oninput: move |e| {
                value.set(e.data().value());
                debounce.action(value()) },
        }
    }
}
