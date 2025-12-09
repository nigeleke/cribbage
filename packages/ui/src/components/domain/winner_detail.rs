use api::dto::{PlayerDTO, UserGameDTO};
use dioxus::prelude::*;

#[component]
pub fn WinnerDetail() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();
    let winner = use_memo(move || game.read().winner);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/winner_detail.css")},
        div {
            class: "winner-detail",
            if let Some(winner) = winner() {
                if winner == PlayerDTO::User {
                    img {
                        class: "winner-detail__medal winner-detail__winner",
                        src: asset!("/assets/winner.png")
                    }
                } else {
                    img {
                        class: "winner-detail__medal winner-detail__loser",
                        src: asset!("/assets/loser.png")
                    }
                }
            }
        }
    }
}
