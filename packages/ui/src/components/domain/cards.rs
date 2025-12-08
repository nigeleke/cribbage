use api::dto::CardDTO;
use dioxus::prelude::*;

use crate::components::Card;

#[component]
pub fn Cards(cards: ReadSignal<Vec<CardDTO>>) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/cards.css")},
        div {
            class: "cards",
            for card in cards() {
                Card { card }
            }
        }
    }
}
