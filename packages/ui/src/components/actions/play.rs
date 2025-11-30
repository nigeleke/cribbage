use api::dto::{CardIdDTO, GameIdDTO, PlayActionDTO, PlayerDTO, PlaysDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::{
    components::{WaitingForOpponent, button::Button},
    toast::Toast,
};

#[component]
pub fn PlayAction() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let selected_cards = use_context::<ReadSignal<Vec<CardIdDTO>>>();
    let selected_count = selected_cards.read().len();

    let plays = use_context::<ReadSignal<PlaysDTO>>();

    let next_action = plays().next_action;
    let user_play_turn = next_action == PlayActionDTO::Play(PlayerDTO::User);

    let can_play = if user_play_turn && let Some(cid) = selected_cards.first() {
        selected_count == 1 && plays().legal_plays.contains(&cid)
    } else {
        false
    };

    let mut play_action = use_action(move || async move {
        if let Some(cid) = selected_cards().first() {
            let result = api::action::play_card(*user_id.read(), game_id, cid.clone()).await;
            match result {
                Ok(_) => (),
                Err(ref error) => {
                    Toast::command_error("Play", error.to_string());
                }
            }
            result
        } else {
            Ok(())
        }
    });

    let on_play = move |_| play_action.call();

    rsx! {
        if let PlayActionDTO::Play(player) = next_action {
            if player == PlayerDTO::User {
                Button {
                    onclick: on_play,
                    disabled: !can_play,
                    "Play"
                }
            } else {
                WaitingForOpponent {}
            }
        }

    }
}
