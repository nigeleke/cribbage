use dioxus::prelude::*;

use crate::components::{AvailableGamesList, HostGameAction, PlayComputerAction};

#[component]
pub fn HomePage() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/home_page.css") }
        div {
            class: "home-page",
            NewGameSection {}
            JoinGameSection {},
        }
    }
}

#[component]
fn NewGameSection() -> Element {
    rsx! {
        section {
            class: "home-page__new-game-section",
            h2 { "Start a New Game" }
            div {
                class: "home-page__new-game__actions",
                HostGameAction {}
                PlayComputerAction {}
            }
        }
    }
}

#[component]
fn JoinGameSection() -> Element {
    rsx! {
        section {
            class: "home-page__join-game-section",
            h2 { "Join a Game" }
            AvailableGamesList {}
        }
    }
}
