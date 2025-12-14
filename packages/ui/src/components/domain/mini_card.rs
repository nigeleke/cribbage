use api::dto::{CardDTO, CardIdDTO};
use dioxus::prelude::*;

#[component]
pub fn MiniCard(card: CardDTO) -> Element {
    rsx! {
        if let CardDTO::FaceUp { cid } = card {
            MiniIdCard { cid }
        }
    }
}

#[component]
pub fn MiniIdCard(cid: CardIdDTO) -> Element {
    let mut chars = cid.chars();
    let rank = chars.next().unwrap_or('?');
    let suit = chars.next().unwrap_or('?');
    let symbol = match suit {
        'H' => '\u{2665}',
        'C' => '\u{2663}',
        'D' => '\u{2666}',
        'S' => '\u{2660}',
        other => other,
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/mini_card.css")},
        div {
            class: "mini-card mini-card__{suit}",
            div { "{rank}" }
            div { class: "mini-card__suit", "{symbol}" }
        },
    }
}
