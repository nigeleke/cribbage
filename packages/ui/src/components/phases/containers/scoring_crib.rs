use dioxus::prelude::*;

use crate::components::{
    Cut, InProgress, OpponentCrib, Scoreboard, ScoringDetail, StartNextRoundAction, UserCrib,
};

#[component]
pub fn ScoringCrib() -> Element {
    rsx! {
        InProgress {
            north: rsx! { UserCrib { StartNextRoundAction {} } },
            south: rsx! { OpponentCrib {} },
            east: rsx! { Scoreboard {} },
            west: rsx! { Cut {} },
            centre: rsx! { ScoringDetail {} },
        }
    }
}
