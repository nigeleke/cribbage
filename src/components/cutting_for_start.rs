use crate::components::Context;
use crate::components::card::Card;

use crate::services::{redraw, start};
use crate::types::*;
use crate::view::{CardSlot, Cut, Cuts, Role};

use leptos::*;
use thaw::*;

/// The Cuts component shows the initial cuts at the start of the game.
/// It enables the user to start or redraw as appropriate to the cuts' ranks.
#[component]
pub fn CuttingForStart(

    cuts: Cuts

) -> impl IntoView {

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
        <Space vertical=true align=SpaceAlign::Center>
            <Space justify=SpaceJustify::SpaceBetween>
                <CardWithLabel card=CardSlot::FaceUp(player_cut) label="Your cut".into() />
                <CardWithLabel card=CardSlot::FaceUp(opponent_cut) label="Opponent's cut".into() />
            </Space>
            { if player_rank == opponent_rank {
                view! { <Button on_click=on_redraw>{start_status}</Button>}
            } else {
                view! { <Button on_click=on_start>{start_status}</Button>}
            }
            }
        </Space>
    }
}

#[component]
fn CardWithLabel(
    card: CardSlot,
    label: String,
) -> impl IntoView {
    view! {
        <Space vertical=true align=SpaceAlign::Center>
            <Card card />
            <span>{label}</span>
        </Space>
    }
}