use api::dto::{PlayerDTO, UserGameDTO};
use dioxus::prelude::*;

#[component]
pub fn WinnerMedal() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();
    let winner = use_memo(move || game.read().winner);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/winner_medal.css")},
        div {
            class: "winner-medal",
            if let Some(winner) = winner() {
                if winner == PlayerDTO::User {
                    img {
                        class: "winner-medal__image winner-medal__winner",
                        src: asset!("/assets/winner.png")
                    }
                } else {
                    img {
                        class: "winner-medal__image winner-medal__loser",
                        src: asset!("/assets/loser.png")
                    }
                }
            }
        }
    }
}
