use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

#[component]
pub fn Toast(toasts: Signal<Vec<String>>) -> Element {
    let mut visible_toasts = use_signal(Vec::<String>::new);

    use_effect(move || {
        let new_toasts = toasts.read().clone();

        if !new_toasts.is_empty() {
            let count = new_toasts.len();
            visible_toasts.write().extend(new_toasts.clone());

            spawn({
                async move {
                    TimeoutFuture::new(3000).await;
                    toasts.write().drain(0..count);
                    visible_toasts.write().drain(0..count);
                }
            });
        }
    });

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/css/toast.css") }
        div {
            class: "toast",
            for toast in visible_toasts.read().iter() {
                div {
                    class: "toast-item",
                    "{toast}"
                }
            }
        }
    }
}
