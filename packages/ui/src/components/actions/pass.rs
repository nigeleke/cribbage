use api::dto::{GameIdDTO, PlayActionDTO, PlayerDTO, PlaysDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::components::{WaitingForOpponent, button::Button};

#[component]
pub fn PassAction() -> Element {
    let plays = use_context::<ReadSignal<PlaysDTO>>();

    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let mut pass_action = use_action(move || async move {
        let result = api::action::pass(*user_id.read(), game_id).await;
        match result {
            Ok(_) => (),
            Err(error) => {
                warn!("{error:?}");
                todo!()
            }
        }
        result
    });

    let on_pass = move |_| pass_action.call();

    rsx! {
        if let PlayActionDTO::Pass(player) = plays().next_action {
            if player == PlayerDTO::User {
                Button {
                    onclick: on_pass,
                    "Pass"
                }
            } else {
                WaitingForOpponent {}
            }
        }
    }
}
