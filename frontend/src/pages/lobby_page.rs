use api::{UnstartedGameId, fetch_unstarted_game};
use dioxus::prelude::*;

#[component]
pub fn LobbyPage(id: UnstartedGameId) -> Element {
    let fetch_game = use_resource(move || async move { fetch_unstarted_game(id).await });
    let mut game = use_signal(|| None);

    use_effect(move || {
        if let Some(result) = fetch_game() {
            game.set(result.ok());
        };
    });

    rsx! {
        if let Some(game) = game() {
            div { "The game {game.name()} is waiting for an opponent" }
        } else {
            div { "Loading..." }
        }
    }
}
