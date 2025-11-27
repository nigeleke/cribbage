use api::dto::{PendingDTO, UserGameDTO};
use dioxus::prelude::*;

use crate::components::WaitingForOpponent;

#[component]
pub fn Confirmation(children: Element) -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();

    rsx! {
        if game.read().pending == PendingDTO::User {
            {children}
        } else if game.read().pending == PendingDTO::Opponent {
            WaitingForOpponent {}
        }
    }
}
