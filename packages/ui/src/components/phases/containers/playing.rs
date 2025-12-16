use dioxus::prelude::*;

use crate::components::{
    CribAndCut, InProgress, OpponentHand, PlayingControls, Plays, Scoreboard, UserHand,
};

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
