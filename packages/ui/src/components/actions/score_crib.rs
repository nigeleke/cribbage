use api::dto::{GameIdDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::{
    components::{Confirmation, button::Button},
    toast::Toast,
};

#[component]
pub fn ScoreCribAction() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let mut score_action = use_action(move || async move {
        let result = api::action::score_crib(*user_id.read(), game_id).await;
        match result {
            Ok(_) => {}
            Err(ref error) => Toast::command_error("Score crib", error.to_string()),
        };
        result
    });

    let on_score = move |_| score_action.call();

    rsx! {
        Confirmation {
            Button {
                onclick: on_score,
                "Score Crib"
            }
        }
    }
}
