use api::dto::{GameIdDTO, PlayerDTO, UserGameDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::{
    components::{Confirmation, button::Button},
    toast::Toast,
};

#[component]
pub fn StartNextRoundAction() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let game = use_context::<ReadSignal<UserGameDTO>>();

    let user_is_currently_dealer = game().dealer == Some(PlayerDTO::User);

    let mut score_action = use_action(move || async move {
        let result = api::action::start_next_round(*user_id.read(), game_id).await;
        match result {
            Ok(_) => {}
            Err(ref error) => Toast::command_error("Start next round", error.to_string()),
        };
        result
    });

    let on_score = move |_| score_action.call();

    rsx! {
        Confirmation {
            Button {
                onclick: on_score,
                if user_is_currently_dealer {
                    "Continue"
                } else {
                    "Deal"
                }
            }
        }
    }
}
