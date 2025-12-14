use api::dto::PlayerDTO;
use dioxus::prelude::*;

use crate::components::{Cut, InProgress, Scoreboard, WinnerCards, WinnerMedal};

#[component]
pub fn Finished() -> Element {
    rsx! {
        InProgress {
            north: rsx! { WinnerCards { player: PlayerDTO::User } },
            south: rsx! { WinnerCards { player: PlayerDTO::Opponent } },
            east: rsx! { Scoreboard {} },
            west: rsx! { Cut {} },
            centre: rsx! { WinnerMedal {} },
        }
    }
}
