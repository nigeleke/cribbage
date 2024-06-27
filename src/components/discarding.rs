use crate::constants::*;
use crate::components::Context;
use crate::components::cards::Cards;
use crate::services::discard;
use crate::view::{CardSlot, Hand};

use leptos::*;
use thaw::*;

#[component]
pub fn Discarding(
    
    current_player_hand: Hand

) -> impl IntoView {

    let (current_player_cards, _) = create_signal(current_player_hand);

    let (selected, set_selected) = create_signal(Vec::<bool>::new());
    let selected_count = move || selected().into_iter().filter(|s| *s).count();
    let selected_cards = move || {
        selected()
            .into_iter()
            .zip(current_player_cards())
            .filter_map(|(s, c)| {
                if let CardSlot::FaceUp(card) = c {
                    s.then_some(card)
                } else {
                    None
                }
        })
        .collect::<Vec<_>>()
    };
    let disabled = (move || selected_count() != CARDS_DISCARDED_TO_CRIB).into_signal();

    let context = use_context::<Context>().unwrap();

    let on_discard = {
        let context = context.clone();
        move |_| {
            let id = context.id.clone();
            let state = context.state;
            let cards = selected_cards().clone();
            spawn_local(async move {
                if let Ok(game) = discard(id, cards).await {
                    state.set(Some(game.clone()));
                }
            });
        }
    };

    view! {
        <Space vertical=true align=SpaceAlign::Center>
            {move || {
                let current_player_cards = current_player_cards();
                view!{ <Cards cards=current_player_cards on_selected=set_selected /> }
            }}
            <Button on_click=on_discard disabled>"Discard"</Button>
        </Space>
    }    
}