use crate::components::cards::Cards;
use crate::components::Context;
use crate::services::prelude::{score_dealer};
use crate::view::prelude::Hand;

use leptos::*;
use style4rs::style;

#[component]
pub fn Scoring(

    cards: Hand,

) -> impl IntoView {
    logging::log!("component::Scoring");

    let class = style!{
        div {
            display: flex;
            flex-direction: column;
            justify-content: space-around;
            align-items: center;
            gap: 5vh;
        }
    };

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
        class = class,
        <div>
            {move || {
                let cards = cards();
                view!{ <Cards cards /> }
            }}
            <span><button on:click=on_score_next>"Score next"</button></span>
        </div>
    }    
}

