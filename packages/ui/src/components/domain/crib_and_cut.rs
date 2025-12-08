use api::dto::{CardDTO, PlayerDTO, UserGameDTO};
use dioxus::prelude::*;

use crate::components::Card;

#[component]
pub fn CribAndCut() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();

    let dealer = use_memo(move || game().dealer);
    let crib = use_memo(move || game().crib);

    let user_crib =
        use_memo(move || dealer().and_then(|dealer| (dealer == PlayerDTO::User).then(&*crib)));

    let opponent_crib =
        use_memo(move || dealer().and_then(|dealer| (dealer == PlayerDTO::Opponent).then(&*crib)));

    let starter_cut = use_memo(move || game().starter_cut);

    rsx! {
        InnerCribAndCut { user_crib, starter_cut, opponent_crib }
    }
}

#[component]
pub fn Cut() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();

    let starter_cut = use_memo(move || game().starter_cut);

    rsx! {
        InnerCribAndCut { user_crib: None, starter_cut, opponent_crib: None }
    }
}

#[component]
fn InnerCribAndCut(
    user_crib: ReadSignal<Option<Vec<CardDTO>>>,
    starter_cut: ReadSignal<Option<CardDTO>>,
    opponent_crib: ReadSignal<Option<Vec<CardDTO>>>,
) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/crib_and_cut.css")},
        div {
            class: "crib-and-cut",
            div {
                class: "crib-and-cut__user-crib",
                PlayerCrib { cards: user_crib }
            }
            div {
                class: "crib-and-cut__cut",
                CutView { starter_cut }
            }
            div {
                class: "crib-and-cut__opponent-crib",
                PlayerCrib { cards: opponent_crib }
            }
        }
    }
}

#[component]
fn PlayerCrib(cards: ReadSignal<Option<Vec<CardDTO>>>) -> Element {
    rsx! {
        if let Some(cards) = cards() {
            Card { card: cards.first().cloned() }
        }
    }
}

#[component]
fn CutView(starter_cut: ReadSignal<Option<CardDTO>>) -> Element {
    rsx! {
        if let Some(card) = starter_cut() {
            Card { card }
        }
    }
}
