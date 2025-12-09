use api::dto::UserGameDTO;
use dioxus::prelude::*;

#[component]
pub fn WinnerDetail() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();
    let winner = use_memo(move || game.read().winner);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/winner_detail.css")},
        div {
            class: "winner-detail",
            // if let Some(winner) = winner() {
                p { "{winner:?}" }
            // }
        }
    }
}
