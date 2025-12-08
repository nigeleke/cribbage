use api::dto::{PlayerDTO, UserGameDTO};
use dioxus::prelude::*;

use crate::components::Cards;

/// The `OpponentCrib` component shows a set of cards in the order provided.
#[component]
pub fn OpponentCrib() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();
    let cards = use_memo(move || {
        if game().dealer == Some(PlayerDTO::Opponent) {
            game().crib
        } else {
            Vec::default()
        }
    });

    rsx! {
        Cards { cards }
    }
}
