use crate::view::Game as GameView;

use super::cuts::Cuts;

use leptos::*;
use style4rs::style;

#[component]
pub fn Game(
    #[prop()]
    game: GameView,
) -> impl IntoView {
    let class = style!  {
        div {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            gap: 1vw;
        }
    };
    
    view! {
        class = class,
        <div>
            <Scoreboard game=game.clone() />
            <PlayArea game=game.clone() />
            <Deck game=game />
        </div>
    }
}

#[component]
fn Scoreboard(
    #[prop()]
    game: GameView
) -> impl IntoView {
    view! { <span>"Scoreboard"</span> }
}

#[component]
fn PlayArea(
    #[prop()]
    game: GameView
) -> impl IntoView {
    match game {
        GameView::Starting(player_cut, opponent_cut) => view!{
            <Cuts player_cut=player_cut opponent_cut=opponent_cut />
        },
    }
}

#[component]
fn Deck(
    #[prop()]
    game: GameView
) -> impl IntoView {
    view! { <span>"deck"</span> }
}
