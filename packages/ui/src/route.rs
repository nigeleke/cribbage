use api::dto::{GameIdDTO, UserIdDTO};
use dioxus::prelude::*;
use dioxus_sdk::storage::*;

use crate::{components::toast::ToastProvider, pages::*};

/// Application routing definitions.
///
/// This enum is used by `dioxus-router` to map URL paths to component
/// screens. Each variant corresponds to a page in the application and
/// defines any dynamic path parameters required to construct that page.
#[derive(Clone, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    /// [`Layout`] component for all pages.
    #[layout(Layout)]

    /// The root page of the application.
    ///
    /// This displays the primary landing UI and is wrapped in the global
    #[route("/")]
    HomePage {},

    /// The game page.
    ///
    /// This route renders the UI for interacting with an existing game.
    /// The `game_id` parameter is extracted from the path segment and
    /// passed into the page component as a [`GameIdDTO`].
    ///
    /// Path: `/game/:game_id`
    #[route("/game/:game_id")]
    GamePage {
        /// The game_id to be viewed.
        game_id: GameIdDTO
    },

    /// Fallback for unknown routes.
    ///
    /// Any unmatched or malformed path is captured here. All remaining
    /// path segments are collected into `segments` for diagnostic or
    /// display purposes.
    ///
    /// Path pattern: `/:..segments`
    #[route("/:..segments")]
    NotFoundPage {
        /// Unused parts of the request path
        segments: Vec<String>
    },
}

#[component]
fn Layout() -> Element {
    let user_id = use_persistent("user_id", UserIdDTO::new);
    let display_user_id = use_memo(move || user_id.read().short_name());
    provide_context(user_id);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/dx-components-theme.css") }
        document::Stylesheet { href: asset!("/assets/css/main.css") }
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico"), type: "image/x-icon" }
        document::Script { r#type: "module", src: asset!("/assets/js/listen_unhandled_promises.js") }

        header { h1 { "Cribbage"  } }
        main {
            ErrorBoundary {
                handle_error: |errors| rsx! { UnexpectedErrorPage { errors } },
                ToastProvider {
                    Outlet::<Route> {}
                }
            }
        }
        footer { "{display_user_id}" " - Copyright © 2025; Nigel Eke. All rights reserved." }
    }
}
