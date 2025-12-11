use std::time::Duration;

use api::dto::AvailableGameEventDTO;
use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, ToastType, consume_toast};

/// Helper for displaying toast notifications within the UI.
///
/// This utility provides a consistent interface for raising user-facing
/// toast messages through the application's toast API. It centralizes
/// formatting, duration choices, and toast types so that components can
/// emit notifications without duplicating boilerplate.
///
/// All methods delegate to a private wrapper around
/// [`consume_toast()`], which retrieves the active toast handle and
/// displays the configured toast.
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

    /// Displays an error toast corresponding to a failed user command.
    ///
    /// This is intended for surfacing errors triggered by client-side
    /// actions (button clicks, form submissions, etc.) and logs the error
    /// via `warn!` before showing a toast.
    ///
    /// The toast is non-permanent and remains visible for 30 seconds.
    pub fn command_error(command: &str, error: String) {
        warn!("{error}");
        Self::toast(
            command,
            &error,
            ToastType::Error,
            Duration::from_secs(30),
            false,
        );
    }

    /// Displays a notification in response to a change in game availability.
    ///
    /// This is typically called when the application receives an
    /// [`AvailableGameEventDTO`] from the server. The toast indicates
    /// whether a game was created or removed.
    ///
    /// The toast is informational and expires after 10 seconds.
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

    /// Displays a server-side error notification.
    ///
    /// Intended for situations where a backend service returns an error
    /// or cannot be reached. These are treated as higher-severity issues
    /// and are shown as warning toasts with a longer, permanent duration.
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
