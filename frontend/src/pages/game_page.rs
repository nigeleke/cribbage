use crate::components::CardFace;
use api::{
    ActiveGameId, Card, CardState, GameState, PlayerState, Plays, Role, UserId, fetch_game_state,
};
use dioxus::prelude::*;

#[component]
pub fn GamePage(id: ActiveGameId) -> Element {
    let user_id = use_context::<Signal<UserId>>();
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
fn ActiveGame(state: GameState) -> Element {
    if let GameState::Starting {
        user_cut,
        opponent_cut,
    } = state
    {
        rsx! { Starting { user_cut, opponent_cut} }
    } else if let GameState::InProgress {
        user_state,
        opponent_state,
        crib,
        cut,
        plays,
        winner,
    } = state
    {
        rsx! { InProgress { user_state, opponent_state, crib, cut, plays, winner }}
    } else {
        rsx! { p {"Unknown"} }
    }
}

#[component]
fn Starting(user_cut: Card, opponent_cut: Card) -> Element {
    rsx! {
        div {
            class: "game-page",
            class: "starting",
            CardFace { card: user_cut }
            CardFace { card: opponent_cut }
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
    rsx! { p {"In progress"} }
}
