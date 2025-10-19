use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use dto::{GameIdDTO, UserIdDTO};

use crate::pages::*;

#[derive(Clone, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    HomePage {},
    #[route("/lobby/:game_id")]
    LobbyPage { game_id: GameIdDTO },
    #[route("/game/:game_id")]
    GamePage { game_id: GameIdDTO },
    #[route("/error?:error")]
    OopsPage { error: String },
    #[route("/:..segments")]
    NotFoundPage { segments: Vec<String> },
}

#[component]
fn Layout() -> Element {
    let user_id = use_persistent("user_id", || UserIdDTO::new());
    provide_context(user_id);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/main.css") }
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico"), type: "image/x-icon" }
        document::Script { r#type: "module", src: asset!("/assets/js/listen_unhandled_promises.js") }

        header { h1 { "Cribbage" } }
        main {
            ErrorBoundary {
                handle_error: |errors| rsx! { ErrorPage { errors } },
                Outlet::<Route> {}
            }
        }
        footer { "Copyright © 2025; Nigel Eke. All rights reserved." }
    }
}
