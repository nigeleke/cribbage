use crate::components::CardFace;
use api::{
    Card, CardState, GameId, PlayerState, Plays, Role, UserGameState, UserId, fetch_game_state,
    redraw, start,
};
use dioxus::prelude::*;

#[component]
pub fn GamePage(id: GameId) -> Element {
    let user_id = use_context::<Signal<UserId>>();
    provide_context(id);

    let mut state = use_signal(|| None);

    let fetch_state = use_resource(move || fetch_game_state(id, user_id()));
    use_effect(move || {
        if let Some(result) = fetch_state() {
            state.set(result.ok())
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/game_page.css") }
        if let Some(state) = state() {
            ActiveGame{ state }
        } else {
            div {
                class: "game-page",
                "Loading..."
            }
        }
    }
}

#[component]
fn ActiveGame(state: UserGameState) -> Element {
    match state {
        UserGameState::Starting {
            user_cut,
            opponent_cut,
            dealer,
        } => {
            rsx! { Starting { user_cut, opponent_cut, dealer } }
        }
        UserGameState::InProgress {
            user_state,
            opponent_state,
            crib,
            cut,
            plays,
            winner,
        } => {
            rsx! { InProgress { user_state, opponent_state, crib, cut, plays, winner }}
        }
    }
}

#[component]
fn Starting(user_cut: Card, opponent_cut: Card, dealer: Option<Role>) -> Element {
    let user_id = use_context::<Signal<UserId>>();
    let game_id = use_context::<GameId>();

    let mut waiting = use_signal(|| false);

    let on_start = move |_| {
        spawn(async move {
            match start(game_id, user_id()).await {
                Ok(ready) => {
                    if !ready {
                        waiting.set(true);
                    }
                }
                Err(e) => panic!("start game failed: {}", e.to_string()),
            }
        });
    };

    let on_redraw = move |_| {
        spawn(async move {
            match redraw(game_id, user_id()).await {
                Ok(ready) => {
                    if !ready {
                        waiting.set(true);
                    }
                }
                Err(e) => panic!("redraw game failed: {}", e.to_string()),
            }
        });
    };

    rsx! {
        div {
            class: "game-page",
            div {
                class: "starting",
                CardFace { card: user_cut }
                CardFace { card: opponent_cut }
            }
            if let Some(dealer) = dealer {
                if dealer == Role::User {
                    h2 { "You deal" }
                } else {
                    h2 { "Opponent deals" }
                }
                button {
                   onclick: on_start,
                   "Ok"
                }
            } else {
                button {
                   onclick: on_redraw,
                   "Redraw"
                }
            }
        }
    }
}

#[component]
fn InProgress(
    user_state: PlayerState,
    opponent_state: PlayerState,
    crib: Vec<CardState>,
    cut: Option<Card>,
    plays: Option<Plays>,
    winner: Option<Role>,
) -> Element {
    rsx! {
        div {
            class: "in-progress",
            div {
                class: "scoreboard",
                h2 { class: "scoreboard-title", "Scoreboard" }
            }
            div {
                class: "card-container",
                h3 { class: "section-title", "Your Hand" }
            }
            div {
                class: "middle-section",
                p { "middle" }
            }
            div {
                class: "card-container",
                h3 { class: "section-title", "Opponent Hand" }
            }
            div {
                class: "crib-cut-container",
                h3 { class: "section-title", "Crib / cut" }
            }
        }
    }
}
