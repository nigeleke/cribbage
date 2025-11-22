use api::UserGameDTO;
use dioxus::prelude::*;

#[component]
pub fn InLobby() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();
    let name = use_memo(move || game().name);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/in_lobby.css") }
        div {
            class: "in-lobby",
            p {
                "The game "
                span { class: "game-name", "{name}" }
                " is waiting for an opponent"
            }
        }
    }
}
