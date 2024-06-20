use super::card::Card;
use super::cards::Cards;
use super::crib::Crib;
use super::cutting_for_start::CuttingForStart;
use super::discarding::Discarding;
use super::playing::Playing;
use super::plays::Plays;
use super::scoreboard::Scoreboard;
use super::scoring::Scoring;

use crate::view::prelude::{
    Card as CardView,
    CardSlot,
    Crib,
    Cuts as CutsView,
    Game as GameView,
    Hand,
    Hands,
    PlayState,
    Role, 
    Peggings
};

use leptos::*;
use style4rs::style;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ScoringEntity {
    Pone,
    Dealer,
    Crib,
}

/// The Game component shows the current game state.
#[component]
pub fn Game(

    game: GameView,

) -> impl IntoView {
    logging::log!("component::Game");

    match game {
        GameView::Starting(cuts) =>
            view! { <StartingGame cuts /> },
        
        GameView::Discarding(scores, hands, crib, dealer) =>
            view! { <DiscardingGame scores hands crib dealer /> },

        GameView::Playing(scores, hands, play_state, cut, crib, dealer) =>
            view! { <PlayingGame scores hands play_state cut crib dealer /> },

        GameView::ScoringPone(scores, dealer, hands, cut, crib) =>
            view! { <ScoringGame scores entity=ScoringEntity::Pone hands cut crib dealer /> },

        GameView::ScoringDealer(scores, dealer, hands, cut, crib) =>
            view! { <ScoringGame scores entity=ScoringEntity::Dealer hands cut crib dealer /> },

        GameView::ScoringCrib(scores, dealer, hands, cut, crib) =>
            view! { <ScoringGame scores entity=ScoringEntity::Crib hands cut crib dealer /> },

        GameView::Finished(scores) =>
            view! { <FinishingGame scores /> },
    }
}

#[component]
fn StartingGame(

    cuts: CutsView,

) -> impl IntoView {
    logging::log!("component::StartingGame");

    let player_view = Box::new(move || {
        let cuts = cuts.clone();
        view! { <CuttingForStart cuts /> }.into()
    });

    view! {
        <Template player_view />
    }
}

#[component]
fn DiscardingGame(

    scores: Peggings,
    hands: Hands,
    crib: Crib,
    dealer: Role,

) -> impl IntoView {
    logging::log!("component::DiscardingGame");

    let opponent_hand = hands[&Role::Opponent].clone();

    let player_view = Box::new(move || {
        let hands = hands.clone();
        let current_player_hand = hands[&Role::CurrentPlayer].clone();
        view! { <Discarding current_player_hand /> }.into()
    });
    
    view! {
        <Template scores player_view opponent_hand dealer cut=CardSlot::FaceDown crib />
    }
}

#[component]
fn PlayingGame(

    scores: Peggings,
    hands: Hands,
    play_state: PlayState,
    cut: CardView,
    crib: Crib,
    dealer: Role,
    
) -> impl IntoView {
    logging::log!("component::PlayingGame");

    let opponent_hand = hands[&Role::Opponent].clone();
    let state = play_state.clone();

    let player_view = Box::new(move || {
        let current_player_hand = hands[&Role::CurrentPlayer].clone();
        let play_state = play_state.clone();
        view! { <Playing current_player_hand play_state /> }.into()
    });

    view! {
        <Template scores player_view opponent_hand dealer cut=CardSlot::FaceUp(cut) crib>
            <Plays state />
        </Template>
    }
}

#[component]
fn ScoringGame(

    scores: Peggings,
    hands: Hands,
    cut: CardView,
    crib: Crib,
    dealer: Role,
    entity: ScoringEntity,
    
) -> impl IntoView {
    logging::log!("component::ScoringGame");

    let cards_being_scored = match entity {
        ScoringEntity::Pone => hands[&dealer.opponent()].clone(),
        ScoringEntity::Dealer => hands[&dealer].clone(),
        ScoringEntity::Crib => crib.clone(),
    };

    let player_hand = hands[&Role::CurrentPlayer].clone();
    let opponent_hand = hands[&Role::Opponent].clone();

    #[allow(unused_braces)]
    let player_view = Box::new(move || {
        view! { <><Scoring cards=player_hand /></> }
    });

    view! {
        <Template scores 
            player_view 
            opponent_hand
            dealer
            cut=CardSlot::FaceUp(cut)
            crib={if entity == ScoringEntity::Crib { Crib::new() } else { crib } }>
            <Cards cards=cards_being_scored />
        </Template>
    }
}

#[component]
fn FinishingGame(

    scores: Peggings,

) -> impl IntoView {
    logging::log!("component::FinishingGame");

    #[allow(unused_braces)]
    let player_view = Box::new(move || {
        view! { <><button>"Done"</button></> }
    });

    view! {
        <Template player_view>
            <Scoreboard scores />
        </Template>
    }
}

#[component]
fn Template(

    player_view: Children,

    #[prop(optional)]
    scores: Peggings,

    #[prop(optional)]
    opponent_hand: Option<Hand>,

    #[prop(optional)]
    dealer: Option<Role>,

    #[prop(optional)]
    cut: CardSlot,

    #[prop(optional)]
    crib: Crib,

    #[prop(optional)]
    children: Option<Children>,

) -> impl IntoView {
    let class = style!{
        div {
            display: flex;
            flex-direction: row;
            justify-content: space-evenly;
        }
        .dynamicview {
            flex-grow: 1;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            align-items: center;
        }
        .cribandcutview {
            display: flex;
            flex-direction: column;
            justify-content: space-between;
        }
    };

    let hide_player_crib = dealer != Some(Role::CurrentPlayer);
    let hide_opponent_crib = dealer != Some(Role::Opponent);

    view!{
        class = class,
        <div>
            <Scoreboard scores=scores />
            <div class="dynamicview">
                { player_view() }
                <div>{children.map(|c| c())}</div>
                { opponent_hand.map(|cards| view! { <Cards cards /> }) }
            </div>
            <div class="cribandcutview">
                {
                    if hide_player_crib {
                        view! { <Card card=CardSlot::Empty /> }
                    } else {
                        view! { <Crib crib=crib.clone() /> }
                    }
                }
                <Card card=cut />
                {
                    if hide_opponent_crib {
                        view! { <Card card=CardSlot::Empty /> }
                    } else {
                        view! ( <Crib crib=crib.clone() />)
                    }
                }
            </div>
        </div>
    }
}
