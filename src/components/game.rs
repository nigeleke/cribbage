use super::card::Card;
use super::cards::Cards;
use super::crib::Crib;
use super::cutting_for_start::CuttingForStart;
use super::discarding::Discarding;
use super::playing::Playing;
use super::plays::Plays;
use super::scoreboard::Scoreboard;
use super::scoring::Scoring;

use crate::view::{
    Card as CardView,
    CardSlot,
    Crib as CribView,
    Cuts as CutsView,
    Game as GameView,
    Hand,
    Hands,
    PlayState,
    Role, 
    Peggings,
};

use leptos::*;
use leptos_meta::*;
use thaw::*;

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
    crib: CribView,
    dealer: Role,

) -> impl IntoView {

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
    crib: CribView,
    dealer: Role,
    
) -> impl IntoView {

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
    crib: CribView,
    dealer: Role,
    entity: ScoringEntity,
    
) -> impl IntoView {

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
            crib={if entity == ScoringEntity::Crib { CribView::new() } else { crib } }>
            <Cards cards=cards_being_scored />
        </Template>
    }
}

#[component]
fn FinishingGame(

    scores: Peggings,

) -> impl IntoView {

    #[allow(unused_braces)]
    let player_view = Box::new(move || {
        view! { <><Button>"Done"</Button></> }
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
    crib: CribView,

    #[prop(optional)]
    children: Option<Children>,

) -> impl IntoView {

    let hide_player_crib = dealer != Some(Role::CurrentPlayer);
    let hide_opponent_crib = dealer != Some(Role::Opponent);

    view!{
        <Grid cols=5>
            <GridItem><Scoreboard scores=scores /></GridItem>
            <GridItem column=3>
                <Space vertical=true justify=SpaceJustify::SpaceAround align=SpaceAlign::Center>
                    { player_view() }
                    <>{children.map(|c| c())}</>
                    { opponent_hand.map(|cards| view! { <Cards cards /> }) }
                </Space>
            </GridItem>
            <GridItem>
                <Space vertical=true justify=SpaceJustify::SpaceAround align=SpaceAlign::End>
                    <PlayerCribView hidden=hide_player_crib crib=crib.clone() />
                    <Card card=cut />
                    <PlayerCribView hidden=hide_opponent_crib crib />
                </Space>
            </GridItem>
        </Grid>
        <Style>
        ".thaw-grid {
          justify-items: center;
        }
        .thaw-grid-item {
          display: flex;
        }
        .thaw-space {
          flex-grow: 1;
        }"
        </Style>
    }
}

#[component]
fn PlayerCribView(
    hidden: bool,
    crib: CribView,
) -> impl IntoView {
    if hidden {
        view! { <Card card=CardSlot::Empty /> }
    } else {
        view! { <Crib crib /> }
    }
}