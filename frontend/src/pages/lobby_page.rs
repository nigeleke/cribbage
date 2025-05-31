use crate::route::Route;
use api::{StartedGameEvent, UnstartedGameId, fetch_unstarted_game, started_game_stream};
use dioxus::{logger::tracing::warn, prelude::*};
use futures::StreamExt;

#[component]
pub fn LobbyPage(id: UnstartedGameId) -> Element {
    let mut game = use_signal(|| None);

    let fetch_game = use_resource(move || async move { fetch_unstarted_game(id).await });
    use_effect(move || {
        if let Some(result) = fetch_game() {
            game.set(result.ok());
        };
    });

    let navigator = use_navigator();

    let _ = use_resource(move || async move {
        if let Some(game) = game() {
            match started_game_stream(*game.id()).await {
                Ok(stream) => {
                    let mut stream = stream.into_inner();
                    while let Some(event) = stream.next().await {
                        match event {
                            Ok(event) => match event {
                                StartedGameEvent::NewGame(game) => {
                                    let active_game_id = *game.active_game_id();
                                    navigator.replace(Route::GamePage { id: active_game_id });
                                }
                            },
                            Err(e) => {
                                warn!("Stream error: {:?}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to fetch stream: {:?}", e);
                    return;
                }
            }
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/lobby_page.css") }
        if let Some(game) = game() {
            div {
               class: "lobby-page",
               "The game "
               span {
                  class: "game-name",
                  "{game.name()}"
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
