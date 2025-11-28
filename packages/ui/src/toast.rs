use std::time::Duration;

use api::dto::AvailableGameEventDTO;
use dioxus_primitives::toast::{ToastOptions, ToastType, consume_toast};

pub struct Toast;

impl Toast {
    fn toast(
        title: &str,
        description: &str,
        toast_type: ToastType,
        duration: Duration,
        permanent: bool,
    ) {
        let api = consume_toast();
        api.show(
            String::from(title),
            toast_type,
            ToastOptions::new()
                .description(description)
                .duration(duration)
                .permanent(permanent),
        );
    }

    pub fn command_error(command: &str, error: String) {
        Self::toast(
            command,
            &error,
            ToastType::Error,
            Duration::from_secs(30),
            false,
        );
    }

    pub fn available_game(event: AvailableGameEventDTO) {
        let description = match event {
            AvailableGameEventDTO::Created { name, .. } => format!("Created {name}"),
            AvailableGameEventDTO::Removed { name, .. } => format!("Removed {name}"),
        };

        Self::toast(
            "Available game",
            &description,
            ToastType::Info,
            Duration::from_secs(10),
            false,
        );
    }

    pub fn server_error(service: &str, error: String) {
        Self::toast(
            service,
            &error,
            ToastType::Warning,
            Duration::from_secs(30),
            true,
        );
    }
}
