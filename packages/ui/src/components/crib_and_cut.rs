use super::CardView;
use api::{CardDTO, PlayerDTO};
use dioxus::prelude::*;

#[component]
pub fn CribAndCut(dealer: PlayerDTO, cards: Vec<CardDTO>, cut: Option<CardDTO>) -> Element {
    let user_crib = (dealer == PlayerDTO::User).then(|| cards.clone());
    let opponent_crib = (dealer == PlayerDTO::Opponent).then(|| cards.clone());

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/crib_and_cut.css")},
        div {
            class: "crib-container",
            PlayerCrib { cards: user_crib }
            CutView { cut }
            PlayerCrib { cards: opponent_crib }
        }
    }
}

#[component]
fn PlayerCrib(cards: Option<Vec<CardDTO>>) -> Element {
    if let Some(cards) = cards {
        let card = cards.first().cloned();
        rsx! { CardView { card } }
    } else {
        rsx! { p {} }
    }
}

#[component]
fn CutView(cut: Option<CardDTO>) -> Element {
    if let Some(card) = cut {
        rsx! { CardView { card } }
    } else {
        rsx! { p {} }
    }
}
