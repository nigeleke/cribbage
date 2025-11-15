use api::{GameEventDTO, GameIdDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn LobbyPage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

    let navigator = use_navigator();

    let mut game_name = use_signal(|| None);

    let _ = use_resource(move || async move {
        let game = api::view::get_game(*user_id.read(), game_id).await?;
        game_name.set(Some(game.name().clone()));
        dioxus::Ok(())
    });

    let _ = use_resource(move || async move {
        let mut stream = api::stream::user_game_events(*user_id.read(), game_id).await?;
        while let Some(Ok(event)) = stream.next().await {
            match event {
                GameEventDTO::OpponentJoined => {
                    navigator.replace(Route::GamePage { game_id });
                }
                _ => {}
            }
        }
        dioxus::Ok(())
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/lobby_page.css") }
        if let Some(name) = game_name() {
            div {
               class: "lobby-page",
               "The game "
               span {
                  class: "game-name",
                  "{name}"
               }
               " is waiting for an opponent"
            }
        } else {
            div {
                class: "lobby-page",
                "Loading..."
            }
        }
    }
}
