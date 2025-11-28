use std::time::Duration;

use dioxus::prelude::*;
use dioxus_sdk::time::use_debounce;

use crate::components::input::*;

#[component]
pub fn DebouncedInput(
    value: ReadSignal<String>,
    name: String,
    placeholder: Option<String>,
    on_debounced_input: EventHandler<String>,
) -> Element {
    let mut value = use_signal(&*value);

    let mut debounce = use_debounce(Duration::from_millis(300), move |value| {
        on_debounced_input.call(value);
    });

    let oninput = move |e: FormEvent| {
        value.set(e.data().value());
        debounce.action(value());
    };

    rsx! {
        Input {
            value,
            name,
            placeholder,
            oninput,
        }
    }
}
