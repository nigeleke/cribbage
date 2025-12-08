use dioxus::prelude::*;

use crate::components::{
    CribAndCut, InProgress, OpponentHand, ScoreCribAction, Scoreboard, ScoringDetail, UserHand,
};

#[component]
pub fn ScoringDealer() -> Element {
    rsx! {
        InProgress {
            north: rsx! { UserHand { ScoreCribAction {} } },
            south: rsx! { OpponentHand {} },
            east: rsx! { Scoreboard {} },
            west: rsx! { CribAndCut {} },
            centre: rsx! { ScoringDetail {} },
        }
    }
}
