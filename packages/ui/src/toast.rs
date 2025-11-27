use std::time::Duration;

use dioxus_primitives::toast::{ToastOptions, ToastType, consume_toast};

pub struct Toast;

impl Toast {
    fn toast(title: &str, description: &str, toast_type: ToastType, permanent: bool) {
        let api = consume_toast();
        api.show(
            String::from(title),
            toast_type,
            ToastOptions::new()
                .description(description)
                .duration(Duration::from_secs(30))
                .permanent(permanent),
        );
    }

    pub fn command_error(command: &str, error: String) {
        Self::toast(command, &error, ToastType::Warning, false);
    }
}
