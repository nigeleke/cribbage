use api::dto::{AvailabilityDTO, AvailableGameDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::route::Route;

#[component]
pub fn SelectGameAction(game: ReadSignal<AvailableGameDTO>) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

    let mut game_id = use_signal(|| None);

    let navigator = use_navigator();

    use_effect(move || {
        if let Some(game_id) = game_id() {
            navigator.push(Route::GamePage { game_id });
        }
    });

    let mut join_game = use_action(move |id| async move {
        let result = api::action::join_game(user_id(), id).await;
        match result {
            Ok(_) => game_id.set(Some(id)),
            Err(ref error) => {
                warn!("SelectGameAction:error {error:?}");
                // TODO: Toast
            }
        };
        result
    });

    let mut rejoin_game = use_action(move |id| async move {
        game_id.set(Some(id));
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
