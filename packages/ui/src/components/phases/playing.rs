use crate::components::{
    CribAndCut, InProgress, OpponentHand, PlayingControls, Plays, Scoreboard, UserHand,
};
use dioxus::prelude::*;

#[component]
pub fn Playing() -> Element {
    rsx! {
        InProgress {
            north: rsx! { UserHand { PlayingControls {} } },
            south: rsx! { OpponentHand {} },
            east: rsx! { Scoreboard {} },
            west: rsx! { CribAndCut {} },
            centre: rsx! { Plays {} },
        }
    }
}
