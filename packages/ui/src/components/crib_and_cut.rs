use crate::components::Card;
use api::{CardDTO, PlayerDTO};
use dioxus::prelude::*;

#[component]
pub fn CribAndCut(
    dealer: ReadSignal<PlayerDTO>,
    cards: ReadSignal<Vec<CardDTO>>,
    starter_cut: ReadSignal<Option<CardDTO>>,
) -> Element {
    let user_crib = use_memo(move || (*dealer.read() == PlayerDTO::User).then(|| cards()));
    let opponent_crib = use_memo(move || (*dealer.read() == PlayerDTO::Opponent).then(|| cards()));

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/crib_and_cut.css")},
        div {
            class: "crib-container",
            PlayerCrib { cards: user_crib }
            CutView { starter_cut }
            PlayerCrib { cards: opponent_crib }
        }
    }
}

#[component]
fn PlayerCrib(cards: ReadSignal<Option<Vec<CardDTO>>>) -> Element {
    if let Some(cards) = cards() {
        let card = cards.first().cloned();
        rsx! { Card { card } }
    } else {
        rsx! { p {} }
    }
}

#[component]
fn CutView(starter_cut: ReadSignal<Option<CardDTO>>) -> Element {
    if let Some(card) = starter_cut() {
        rsx! { Card { card } }
    } else {
        rsx! { p {} }
    }
}
