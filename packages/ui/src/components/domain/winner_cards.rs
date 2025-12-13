use api::dto::{PlayerDTO, UserGameDTO};
use dioxus::prelude::*;

use crate::components::MiniCards;

#[component]
pub fn WinnerCards(player: PlayerDTO) -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();

    let winner = use_memo(move || game.read().winner == Some(player));

    let hand = use_memo(move || {
        if player == PlayerDTO::User {
            game().user_state.hand
        } else {
            game().opponent_state.hand
        }
    });

    let crib = use_memo(move || (game.read().dealer == Some(player)).then_some(game().crib));

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/winner_cards.css")},
        div {
            class: "winner-cards",
            class: if winner() {"winner-cards__winner"},
            MiniCards { cards: hand() }
            if let Some(cards) = crib() {
                MiniCards { cards }
            }
        }
    }
}
