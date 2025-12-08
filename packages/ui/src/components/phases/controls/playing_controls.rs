use api::dto::{PlayActionDTO, PlaysDTO, UserGameDTO};
use dioxus::prelude::*;

use crate::components::{GoAction, PlayAction, ScorePoneAction};

/// The `PlayingHand` component shows a set of cards (in the order provided).
#[component]
pub fn PlayingControls() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();

    rsx! {
        if let Some(plays) = game().plays {
            InnerPlayingControls { plays }
        }
    }
}

#[component]
fn InnerPlayingControls(plays: ReadSignal<PlaysDTO>) -> Element {
    provide_context(plays);

    let next_action = plays.read().next_action;

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/playing_hand.css")},
        div {
            class: "playing-hand",
            if let PlayActionDTO::Play(_) = next_action {
                PlayAction {}
            } else if let PlayActionDTO::Go(_) = next_action {
                GoAction {}
            } else {
                ScorePoneAction {}
            }
        }
    }
}
