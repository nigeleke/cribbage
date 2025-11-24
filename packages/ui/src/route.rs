use api::dto::{GameIdDTO, UserIdDTO};
use dioxus::prelude::*;
use dioxus_sdk::storage::*;

use crate::pages::*;

#[derive(Clone, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    HomePage {},
    #[route("/game/:game_id")]
    GamePage { game_id: GameIdDTO },
    #[route("/error/")]
    ErrorPage { error: String },
    #[route("/:..segments")]
    NotFoundPage { segments: Vec<String> },
}

#[component]
fn Layout() -> Element {
    let user_id = use_persistent("user_id", || UserIdDTO::new());
    let display_user_id = use_memo(move || user_id.read().short_name());
    provide_context(user_id);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/dx-components-theme.css") }
        document::Stylesheet { href: asset!("/assets/css/main.css") }
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico"), type: "image/x-icon" }
        // document::Script { r#type: "module", src: asset!("/assets/js/listen_unhandled_promises.js") }

        header { h1 { "Cribbage"  } }
        main {
            ErrorBoundary {
                handle_error: |errors| rsx! { UnexpectedErrorPage { errors } },
                Outlet::<Route> {}
            }
        }
        footer { "{display_user_id}" " - Copyright © 2025; Nigel Eke. All rights reserved." }
    }
}
