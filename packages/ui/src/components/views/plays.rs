use crate::components::Card;
use api::{CardDTO, PlayerDTO, UserGameDTO};
use dioxus::prelude::*;

#[component]
pub fn Plays() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/plays.css")},
        if let Some(plays) = game().plays {
            div {
                class: "plays",
                div {
                    class: "plays__play",
                    div {
                        class: "plays__previous",
                        for (player_id, card) in plays.previous {
                            PlayedCard { player_id, card }
                        }
                    }
                    div {
                        class: "plays__current",
                        for (player_id, card) in plays.current {
                            PlayedCard { player_id, card }
                        }
                    }
                }
                h2 {
                    class: "plays__running-total",
                    hidden: plays.running_total == 0,
                    "{plays.running_total}"
                }
            }
        }
    }
}

#[component]
fn PlayedCard(player_id: PlayerDTO, card: CardDTO) -> Element {
    let whom = match player_id {
        PlayerDTO::User => "plays__user",
        PlayerDTO::Opponent => "plays__opponent",
    };

    rsx! {
        div {
            class: "plays__card {whom}",
            Card { card }
        }
    }
}
