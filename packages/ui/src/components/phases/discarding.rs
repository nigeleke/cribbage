use crate::components::{
    CribAndCut, DiscardingControls, InProgress, OpponentHand, Scoreboard, UserHand,
};
use dioxus::prelude::*;

#[component]
pub fn Discarding() -> Element {
    rsx! {
        InProgress {
            north: rsx! { UserHand { DiscardingControls {} } },
            south: rsx! { OpponentHand {} },
            east: rsx! { Scoreboard {} },
            west: rsx! { CribAndCut { } },
            centre: rsx! {},
        }
    }
}
