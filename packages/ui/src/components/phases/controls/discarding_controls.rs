use api::dto::{CardDTO, CardIdDTO, GameIdDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::{components::button::*, route::Route};

#[component]
pub fn DiscardingControls() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let selected_cards = use_context::<ReadSignal<Vec<CardIdDTO>>>();
    let selected_count = selected_cards.read().len();

    let can_discard = selected_count == 2;

    let mut discard_action = use_action(move |cards: Vec<CardIdDTO>| async move {
        let result = api::action::discard_cards_to_crib(*user_id.read(), game_id, cards).await;
        // TODO: Toast errors...
        result
    });

    let on_discard = move |_| {
        let cards = selected_cards();
        discard_action.call(cards);
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/discarding_hand.css")},
        div {
            Button {
                onclick: on_discard,
                disabled: !can_discard,
                "Discard"
            }

            // } else {
            //     WaitingForOpponent {}
            // }
        }
    }
}
