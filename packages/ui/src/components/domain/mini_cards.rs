use api::dto::CardDTO;
use dioxus::prelude::*;

use crate::components::MiniCard;

#[component]
pub fn MiniCards(cards: Vec<CardDTO>) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/mini_cards.css")},
        div {
            class: "mini-cards",
            for card in cards {
                MiniCard { card }
            }
        }
    }
}
