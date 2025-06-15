use crate::pages::*;
use api::{GameId, UserId};
use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    HomePage {},

    #[route("/lobby/:id")]
    LobbyPage { id: GameId },

    #[route("/game/:id")]
    GamePage { id: GameId },

    #[route("/:..segments")]
    NotFoundPage { segments: Vec<String> },
}

#[component]
fn Layout() -> Element {
    let user_id = use_persistent("user_id", UserId::default);
    provide_context(user_id);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/main.css")}
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico"), type: "image/x-icon" }
        document::Script { r#type: "module", src: asset!("/assets/js/listen_unhandled_promises.js") }

        header { h1 {"Cribbage"} }
        main {
            ErrorBoundary {
                handle_error: |errors| rsx! { ErrorPage { errors } },
                Outlet::<Route> {}
            }
        }
        footer { "Copyright © 2025; Nigel Eke. All rights reserved" }
    }
}
