use super::play_area::discarding::Discarding;
use super::playing::Playing;
use super::scoring::Scoring;
use super::starting::Starting;

use crate::view::prelude::Game as GameView;

use leptos::*;

/// Show the main area for the game, depending on context.
#[component]
pub(crate) fn PlayArea() -> impl IntoView {
    let game = use_context::<GameView>().unwrap();
    match game {
        GameView::Starting(cuts) =>
            view!{ <Starting cuts=cuts /> }.into_view(),

        GameView::Discarding(_, hands, _, _) =>
            view!{ <Discarding hands=hands /> }.into_view(),

        GameView::Playing(_, hands, play_state, _, _, _) =>
            view!{ <Playing hands=hands play_state=play_state /> }.into_view(),

        GameView::ScoringPone(_, role, hands, _, _) =>
            view!{ <Scoring role=role hands=hands /> }.into_view(),

        GameView::ScoringDealer(_, role, hands, _, _) =>
            view!{ <Scoring role=role hands=hands /> }.into_view(),

        GameView::ScoringCrib(_, role, hands, _, _) =>
            view!{ <Scoring role=role hands=hands /> }.into_view(),
    }
}
