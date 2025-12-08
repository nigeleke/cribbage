use api::dto::{GameIdDTO, PlayActionDTO, PlayerDTO, PlaysDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::{
    components::{WaitingForOpponent, button::Button},
    toast::Toast,
};

#[component]
pub fn GoAction() -> Element {
    let plays = use_context::<ReadSignal<PlaysDTO>>();

    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let mut go_action = use_action(move || async move {
        let result = api::action::go(*user_id.read(), game_id).await;
        match result {
            Ok(_) => (),
            Err(ref error) => {
                Toast::command_error("Go", error.to_string());
            }
        }
        result
    });

    let on_go = move |_| go_action.call();

    rsx! {
        if let PlayActionDTO::Go(player) = plays().next_action {
            if player == PlayerDTO::User {
                Button {
                    onclick: on_go,
                    "Go"
                }
            } else {
                WaitingForOpponent {}
            }
        }
    }
}
