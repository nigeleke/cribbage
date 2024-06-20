use crate::components::Context;
use crate::components::card::Card;

use crate::services::prelude::{redraw, start};
use crate::types::prelude::*;
use crate::view::prelude::{CardSlot, Cut, Cuts, Role};

use leptos::*;
use style4rs::style;

/// The Cuts component shows the initial cuts at the start of the game.
/// It enables the user to start or redraw as appropriate to the cuts' ranks.
#[component]
pub(crate) fn CuttingForStart(

    cuts: Cuts

) -> impl IntoView {
    logging::log!("component::CuttingForStart");

    let class = style!{
        div {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 5vh;
        }
        .cuts {
            display: flex;
            flex-direction: row;
            gap: 5vw;
        }
        .cut {
            display: flex;
            flex-direction: column;
            gap: 5vh;
        }
        .playercut {
            opacity: 0;
            animation: spin-in 0.75s ease-in forwards;
        }
        .opponentcut {
            opacity: 0;
            animation: spin-in 0.75s ease-in forwards;
            animation-delay: 1s;
        }
        button {
            flex-shrink: 1;
            opacity: 0;
            animation: fade-in 0.25s ease-in forwards;
            animation-delay: 1.5s;
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

    let player_cut: Cut = cuts[&Role::CurrentPlayer];
    let player_rank = player_cut.rank();

    let opponent_cut: Cut = cuts[&Role::Opponent];
    let opponent_rank = opponent_cut.rank();

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
            <div class="cuts">
                <div class="cut">
                    <div class="playercut"><Card card=CardSlot::FaceUp(player_cut) /></div>
                    <div>"Your cut"</div>
                </div>
                <div class="cut">
                    <div class="opponentcut"><Card card=CardSlot::FaceUp(opponent_cut) /></div>
                    <div>"Opponent's cut"</div>
                </div>
            </div>
            { if player_rank == opponent_rank {
                view! { class = class, <button on:click=on_redraw>{start_status}</button>}
            } else {
                view! { class = class, <button on:click=on_start>{start_status}</button>}
            }
            }
        </div>
    }
}
