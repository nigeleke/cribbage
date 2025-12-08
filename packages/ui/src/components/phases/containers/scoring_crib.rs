use dioxus::prelude::*;

use crate::components::{
    CribAndCut, InProgress, OpponentHand, Scoreboard, StartNextRoundAction, UserHand,
};

#[component]
pub fn ScoringCrib() -> Element {
    rsx! {
        InProgress {
            north: rsx! { UserHand { StartNextRoundAction {} } },
            south: rsx! { OpponentHand {} },
            east: rsx! { Scoreboard {} },
            west: rsx! { CribAndCut {} },
            centre: rsx! { "Crib sheet" },
        }
    }
}
