use api::dto::{CardIdDTO, GameIdDTO, PlayActionDTO, PlaysDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::{
    components::{Confirmation, WaitingForOpponent, button::Button},
    toast::Toast,
};

#[component]
pub fn ScorePoneAction() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let plays = use_context::<ReadSignal<PlaysDTO>>();

    let next_action = plays().next_action;
    let current_len = plays().current.len();
    let previous_len = plays().previous.len();

    let mut score_action = use_action(move || async move { dioxus::Ok(()) });

    let on_score = move |_| score_action.call();

    rsx! {
        p { "Current len: {current_len}" }
        p { "Previous len: {previous_len}" }
        if let PlayActionDTO::ScorePone = next_action {
            Confirmation {
                Button {
                    onclick: on_score,
                    "Score Pone"
                }
            }
        }
    }
}
