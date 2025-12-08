use api::dto::{PlayerDTO, UserGameDTO};
use dioxus::prelude::*;

use crate::components::Cards;

/// The `UserCrib` component shows a set of cards in the order provided.
#[component]
pub fn UserCrib(children: Element) -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();
    let cards = use_memo(move || {
        if game().dealer == Some(PlayerDTO::User) {
            game().crib
        } else {
            Vec::default()
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/user_hand.css") },
        div {
            class: "user-hand",
            div {
                class: "user-hand__cards",
                Cards { cards }
            }
            div {
                class: "user-hand__controls",
                {children}
            }
        }
    }
}
