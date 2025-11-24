use crate::components::{WaitingForOpponent, button::Button};
use crate::route::Route;
use api::dto::{CardIdDTO, GameIdDTO, PlayActionDTO, PlayerDTO, PlaysDTO, UserIdDTO};
use dioxus::prelude::*;

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

    let navigator = use_navigator();

    let on_play = move |_| {
        spawn(async move {
            if let Some(cid) = selected_cards().first() {
                match api::action::play_card(*user_id.read(), game_id, cid.clone()).await {
                    Ok(_) => {}
                    Err(error) => {
                        warn!("GamePage:play:error {error:?}");
                        let error = error.to_string();
                        navigator.push(Route::ErrorPage { error });
                    }
                }
            }
        });
    };

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
