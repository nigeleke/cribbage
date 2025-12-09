use dioxus::prelude::*;

use crate::components::{
    Cut, InProgress, OpponentCrib, OpponentHand, Scoreboard, UserCrib, UserHand, WinnerDetail,
};

#[component]
pub fn Finished() -> Element {
    rsx! {
        InProgress {
            north: rsx! { UserHand {} UserCrib {} },
            south: rsx! { OpponentHand {} OpponentCrib {} },
            east: rsx! { Scoreboard {} },
            west: rsx! { Cut {} },
            centre: rsx! { WinnerDetail {} },
        }
    }
}
