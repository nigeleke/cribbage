use api::{GameIdDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn LobbyPage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

    let navigator = use_navigator();

    // let mut game = use_signal(|| None::<UserGameDTO>);

    use_effect(move || {
        // if let Some(game) = game() {
        //     match game.phase() {
        //         Phase::Lobby => {}
        //         _ => {
        //             navigator.replace(Route::GamePage { game_id });
        //         }
        //     }
        // }
    });

    let _ = use_resource(move || async move {
        // let mut stream = api::user_game_stream(*user_id.read(), game_id).await?;
        // while let Some(Ok(updated_game)) = stream.next().await {
        //     game.set(Some(updated_game));
        // }
        dioxus::Ok(())
    });

    let _ = use_resource(move || async move {
        // let initial_game = api::get_game(*user_id.read(), game_id).await?;
        // game.set(initial_game);
        dioxus::Ok(())
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/lobby_page.css") }
        // if let Some(game) = game() {
        //     div {
        //        class: "lobby-page",
        //        "The game "
        //        span {
        //           class: "game-name",
        //           "{game.name()}"
        //        }
        //        " is waiting for an opponent"
        //     }
        // } else {
            div {
                class: "lobby-page",
                "Loading..."
            }
        // }
    }
}
