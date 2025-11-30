use api::dto::{GameIdDTO, PlayActionDTO, PlaysDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::{
    components::{Confirmation, button::Button},
    toast::Toast,
};

#[component]
pub fn ScorePoneAction() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let plays = use_context::<ReadSignal<PlaysDTO>>();

    let next_action = plays().next_action;

    let mut score_action = use_action(move || async move {
        let result = api::action::acknowledge_plays_ended(*user_id.read(), game_id).await;
        match result {
            Ok(_) => {}
            Err(ref error) => Toast::command_error("Acknowledge plays ended", error.to_string()),
        };
        result
    });

    let on_score = move |_| score_action.call();

    rsx! {
        if next_action == PlayActionDTO::ScorePone {
            Confirmation {
                Button {
                    onclick: on_score,
                    "Score Pone"
                }
            }
        }
    }
}
