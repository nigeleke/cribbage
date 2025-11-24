use crate::components::{PassAction, PlayAction};
use api::dto::{PlaysDTO, UserGameDTO};
use dioxus::prelude::*;

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

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/playing_hand.css")},
        div {
            class: "playing-hand",
            PlayAction {}
            PassAction {}
        }
    }
}
