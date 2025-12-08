use dioxus::prelude::*;

#[component]
pub fn MiniCard(card: String) -> Element {
    let mut bytes = card.chars();
    let rank = bytes.next().unwrap_or('?');
    let suit = bytes.next().unwrap_or('?');

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
        }
    }
}
