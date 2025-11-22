use crate::components::WaitingForOpponent;
use crate::route::Route;
use api::{GameIdDTO, PlayActionDTO, PlayerDTO, PlaysDTO, UserIdDTO};
use dioxus::prelude::*;

#[component]
pub fn PassAction() -> Element {
    let plays = use_context::<ReadSignal<PlaysDTO>>();

    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let navigator = use_navigator();

    let on_pass = move |_| {
        spawn(async move {
            match api::action::pass(*user_id.read(), game_id).await {
                Ok(_) => {}
                Err(error) => {
                    warn!("GamePage:pass:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            }
        });
    };

    rsx! {
        if let PlayActionDTO::Pass(player) = plays().next_action {
            if player == PlayerDTO::User {
                button {
                    onclick: on_pass,
                    "Pass"
                }
            } else {
                WaitingForOpponent {}
            }
        }
    }
}
