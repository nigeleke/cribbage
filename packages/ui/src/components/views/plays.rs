use crate::components::Card;
use api::{CardDTO, PlayerDTO, PlaysDTO};
use dioxus::prelude::*;

#[component]
pub fn Plays(plays: ReadSignal<Option<PlaysDTO>>) -> Element {
    if let Some(plays) = plays() {
        let running_total = plays.running_total;

        rsx! {
            document::Stylesheet { href: asset!("/assets/css/plays.css")},
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
                    hidden: running_total == 0,
                    "{running_total}"
                }
            }
        }
    } else {
        rsx! {}
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
