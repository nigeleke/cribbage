use crate::components::cards::Cards;
use crate::components::Context;
use crate::services::{pass, play, score_pone};
use crate::view::{CardSlot, Hand, PlayState};

use leptos::*;
use style4rs::style;

#[component]
pub fn Playing(

    current_player_hand: Hand,
    play_state: PlayState

) -> impl IntoView {
    logging::log!("component::Playing");

    let class = style!{
        div {
            display: flex;
            flex-direction: column;
            justify-content: space-around;
            align-items: center;
            gap: 5vh;
        }
    };

    let (current_player_cards, _) = create_signal(current_player_hand.clone());

    let (legal_plays, _) = create_signal(play_state.legal_plays().clone());

    let (selected, set_selected) = create_signal(Vec::<bool>::new());
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
    let selected_play = move || {
        let cards = selected_cards().into_iter().filter(|c| legal_plays().contains(c)).collect::<Vec<_>>();
        (cards.len() == 1).then(|| cards[0])
    };
    let disabled = move || { selected_play().is_none() };

    let context = use_context::<Context>().unwrap();

    let on_play = {
        let context = context.clone();
        move |_| {
            let id = context.id.clone();
            let state = context.state;
            let selected_play = selected_play();
            let cards = selected_play.unwrap();
            spawn_local(async move {
                if let Ok(game) = play(id, cards).await {
                    state.set(Some(game.clone()));
                }
            });
        }
    };

    let on_pass = {
        let context = context.clone();
        move |_| {
            let id = context.id.clone();
            let state = context.state;
            spawn_local(async move {
                if let Ok(game) = pass(id).await {
                    state.set(Some(game.clone()));
                }
            });
        }
    };

    let on_score_pone = {
        let context = context.clone();
        move |_| {
            let id = context.id.clone();
            let state = context.state;
            spawn_local(async move {
                if let Ok(game) = score_pone(id).await {
                    state.set(Some(game.clone()));
                }
            });
        }    
    };

    view! {
        class = class,
        <div>
            {move || {
                let current_player_cards = current_player_cards();
                view!{ <Cards cards=current_player_cards on_selected=set_selected /> }
            }}
            <span>{
                if play_state.all_cards_are_played() {
                    view! { <button on:click=on_score_pone>"Score pone"</button> }
                } else if play_state.must_pass() {
                    view! { <button on:click=on_pass>"Pass"</button> }
                } else {
                    view! { <button on:click=on_play disabled=disabled>"Play"</button> }
                }
            }</span>
        </div>
    }    
}

