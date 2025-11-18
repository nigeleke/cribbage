use api::{GameIdDTO, PhaseDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn LobbyPage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

    let navigator = use_navigator();

    let mut game = use_memo(|| None);
    let mut game_name = use_memo(|| None);

    let mut game_stream = use_action(move || async move {
        let mut stream = api::stream::user_game_stream(*user_id.read(), game_id).await?;
        while let Some(Ok(updated_game)) = stream.next().await {
            game.set(Some(updated_game));
        }
        dioxus::Ok(())
    });

    let _ = use_resource(move || async move {
        let updated_game = api::view::get_game(*user_id.read(), game_id).await?;
        game_stream.call();
        game.set(Some(updated_game));
        dioxus::Ok(())
    });

    use_effect(move || {
        if let Some(game) = game() {
            game_name.set(Some(game.name));
            if game.phase == PhaseDTO::CuttingForDeal {
                navigator.replace(Route::GamePage { game_id });
            }
        };
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
