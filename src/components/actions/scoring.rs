use crate::components::cards::Cards;
use crate::view::prelude::{CardSlot, Role};

use leptos::*;

pub fn scoring_action(cards: Vec<CardSlot>) -> impl Fn() -> impl IntoView {
    move || {
        let (current_player_cards, _) = create_signal(cards.clone());

        view! {
            <div>
                <div>
                    {move || {
                        let current_player_cards = current_player_cards();
                        view!{ <Cards cards=current_player_cards /> }
                    }}
                    <span><button>"Next"</button></span>
                </div>
            </div>
        }
    }
}
