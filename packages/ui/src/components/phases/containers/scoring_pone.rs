use dioxus::prelude::*;

use crate::components::{
    CribAndCut, InProgress, OpponentHand, ScoreDealerAction, Scoreboard, ScoringDetail, UserHand,
};

#[component]
pub fn ScoringPone() -> Element {
    rsx! {
        InProgress {
            north: rsx! { UserHand { ScoreDealerAction {} } },
            south: rsx! { OpponentHand {} },
            east: rsx! { Scoreboard {} },
            west: rsx! { CribAndCut {} },
            centre: rsx! { ScoringDetail {} },
        }
    }
}
