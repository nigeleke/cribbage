use crate::components::cards::Cards;
use crate::components::Context;
use crate::services::{score_dealer};
use crate::view::Hand;

use leptos::*;
use thaw::*;

#[component]
pub fn Scoring(

    cards: Hand,

) -> impl IntoView {

    let cards = move || cards.clone();

    let context = use_context::<Context>().unwrap();

    let on_score_next = {
        let context = context.clone();
        move |_| {
            let id = context.id.clone();
            let state = context.state;
            spawn_local(async move {
                if let Ok(game) = score_dealer(id).await {
                    state.set(Some(game.clone()));
                }
            });
        }    
    };

    view! {
        <div>
            {move || {
                let cards = cards();
                view!{ <Cards cards /> }
            }}
            <Button on_click=on_score_next>"Score next"</Button>
        </div>
    }    
}

