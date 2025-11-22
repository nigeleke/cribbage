use crate::route::Route;
use api::{AvailableGameDTO, UserIdDTO};
use dioxus::prelude::*;

#[component]
pub fn SelectGameAction(game: ReadSignal<AvailableGameDTO>) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let name = match game() {
        AvailableGameDTO::Lobby { name, .. } => name,
        AvailableGameDTO::Active { name, .. } => name,
    };

    let navigator = use_navigator();

    let mut join_game = use_action(move |game_id| async move {
        match api::action::join_game(user_id(), game_id).await {
            Ok(_) => {
                navigator.push(Route::GamePage { game_id });
            }
            Err(error) => {
                warn!("SelectGameAction:error {error:?}");
                navigator.push(Route::ErrorPage {
                    error: error.to_string(),
                });
            }
        };
        dioxus::Ok(())
    });

    let mut rejoin_game = use_action(move |game_id| async move {
        navigator.push(Route::GamePage { game_id });
        dioxus::Ok(())
    });

    let select_game = |available_game: AvailableGameDTO| {
        move |_| match available_game {
            AvailableGameDTO::Lobby { game_id, .. } => join_game.call(game_id),
            AvailableGameDTO::Active { game_id, .. } => rejoin_game.call(game_id),
        }
    };

    rsx! {
        div {
            class: "select-game-action",
            onclick: select_game(game()),
            span { class: "game-name", "{name}" }
        }
    }
}
