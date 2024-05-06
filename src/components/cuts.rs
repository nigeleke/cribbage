use super::card::Card;

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
    let opponent_card: Card = opponent_cut.into();

    let start_status = 
        if player_card.rank() < opponent_card.rank() {
            "Your deal"
        } else if player_card.rank() > opponent_card.rank() {
            "Opponent deals"
        } else {
            "Redraw"
        };

    let on_click = move |_| {
        let id = use_context::<crate::pages::game_page::GameParams>().unwrap().id;
        if player_card.rank() == opponent_card.rank() {
            spawn_local(async {
                let _ = redraw(id).await;
            });
        } else {
            spawn_local(async {
                let _ = redraw(id).await;
            });
        };

    };        

    view! {
        class = class,
        <div>
            <div>
                <span><Card card=player_cut.into() label="Your cut".into() /></span>
                <span><Card card=opponent_cut.into() label="Opponent".into() /></span>
            </div>
            <button on:click=on_click>{start_status}</button>
        </div>
    }
}
