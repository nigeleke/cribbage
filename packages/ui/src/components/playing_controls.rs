use crate::{components::WaitingForOpponent, route::Route};
use api::{CardIdDTO, GameIdDTO, PlayActionDTO, PlayerDTO, PlaysDTO, UserGameDTO, UserIdDTO};
use dioxus::prelude::*;

/// The `PlayingHand` component shows a set of cards (in the order provided).
#[component]
pub fn PlayingControls() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();

    rsx! {
        if let Some(plays) = game().plays {
            InnerPlayingControls { plays }
        }
    }
}

#[component]
fn InnerPlayingControls(plays: PlaysDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let selected_cards = use_context::<ReadSignal<Vec<CardIdDTO>>>();
    let selected_count = selected_cards.read().len();

    let next_action = plays.next_action;
    let user_play_turn = next_action == PlayActionDTO::Play(PlayerDTO::User);

    let can_play = if user_play_turn && let Some(cid) = selected_cards.first() {
        selected_count == 1 && plays.legal_plays.contains(&cid)
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
        document::Stylesheet { href: asset!("/assets/css/playing_hand.css")},
        div {
            class: "playing-hand",
            if let PlayActionDTO::Play(player) = next_action {
                if player == PlayerDTO::User {
                    button {
                        onclick: on_play,
                        disabled: !can_play,
                        "Play"
                    }
                } else {
                    WaitingForOpponent { }
                }
            } else if let PlayActionDTO::Pass(player) = next_action {
                if player == PlayerDTO::User {
                    button {
                        onclick: on_pass,
                        "Pass"
                    }

                } else {
                    WaitingForOpponent { }
                }
            } else if PlayActionDTO::ScorePone == next_action {
                button { "Score pone" }
            }
        }
    }
}
