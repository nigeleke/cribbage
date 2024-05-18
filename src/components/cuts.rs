use super::card::Card;
use super::prelude::Context;

use crate::services::prelude::{redraw, start};
use crate::view::{Card, OpponentCut, PlayerCut};

use leptos::*;
use style4rs::style;

#[component]
pub fn Cuts(
    #[prop()]
    player_cut: PlayerCut,
    #[prop()]
    opponent_cut: OpponentCut,
) -> impl IntoView {
    let class = style!{
        div {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 5vh;
        }
        div div {
            display: flex;
            flex-direction: row;
            gap: 5vw;
        }
        span:first-child {
            opacity: 0;
            animation: spin-in 1s ease-in forwards;
        }
        span:last-child {
            opacity: 0;
            animation: spin-in 1s ease-in forwards;
            animation-delay: 1s;
        }
        button {
            flex-shrink: 1;
            opacity: 0;
            animation: fade-in 0.2s ease-in forwards;
            animation-delay: 2s;
        }
        @keyframes spin-in {
            from { transform: rotate(0deg); opacity: 0 }
            to { transform: rotate(360deg); opacity: 1 }
        }
        @keyframes fade-in {
            from { opacity: 0 }
            to { opacity: 1 }
        }
    };

    let player_card: Card = player_cut.into();
    let player_rank = player_card.rank();

    let opponent_card: Card = opponent_cut.into();
    let opponent_rank = opponent_card.rank();

    let start_status = match (player_rank, opponent_rank) {
        (pr, or) if pr < or => "Your deal",
        (pr, or) if pr > or => "Opponent deals",
        _ => "Redraw",
    };

    let context = use_context::<Context>().unwrap();

    let on_redraw = {
        let context = context.clone();
        move |_| {
            let id = context.id.clone();
            let state = context.state;
            spawn_local(async move {
                if let Ok(game) = redraw(id).await {
                    state.set(Some(game.clone()));
                }
            });
        }
    };

    let on_start = {
        let context = context.clone();
        move |_| {
            let id = context.id.clone();
            let state = context.state;
            spawn_local(async move {
                if let Ok(game) = start(id).await {
                    state.set(Some(game));
                }
            });
        }
    };

    view! {
        class = class,
        <div>
            <div>
                <span><Card card=player_cut.into() label="Your cut".into() /></span>
                <span><Card card=opponent_cut.into() label="Opponent".into() /></span>
            </div>
            { if player_rank == opponent_rank {
                view! {<button on:click=on_redraw>"Redraw"</button>}
              } else {
                view! {<button on:click=on_start>{start_status}</button>}
              }
            }
            
        </div>
    }
}
