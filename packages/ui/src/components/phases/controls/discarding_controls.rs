use crate::components::button::*;
use crate::route::Route;
use api::dto::{CardIdDTO, GameIdDTO, UserIdDTO};
use dioxus::prelude::*;

#[component]
pub fn DiscardingControls() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let selected_cards = use_context::<ReadSignal<Vec<CardIdDTO>>>();
    let selected_count = selected_cards.read().len();

    let can_discard = selected_count == 2;

    let navigator = use_navigator();

    let on_discard = move |_| {
        spawn(async move {
            match api::action::discard_cards_to_crib(*user_id.read(), game_id, selected_cards())
                .await
            {
                Ok(_) => {}
                Err(error) => {
                    warn!("GamePage:discard_cards_to_crib:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            }
        });
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
