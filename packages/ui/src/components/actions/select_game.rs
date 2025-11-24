use crate::route::Route;
use api::dto::{AvailabilityDTO, AvailableGameDTO, UserIdDTO};
use dioxus::prelude::*;

#[component]
pub fn SelectGameAction(game: ReadSignal<AvailableGameDTO>) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

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
        move |_| match available_game.availability {
            AvailabilityDTO::Public => join_game.call(available_game.game_id),
            AvailabilityDTO::Private => rejoin_game.call(available_game.game_id),
        }
    };

    rsx! {
        div {
            onclick: select_game(game()),
            "{game().name}"
        }
    }
}
