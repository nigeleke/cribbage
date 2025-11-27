use api::dto::UserIdDTO;
use dioxus::prelude::*;

use crate::{
    components::{AvailableGamesList, HostGameAction, PlayComputerAction},
    toast::Toast,
};

#[component]
pub fn HomePage() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

    let _ = use_resource(move || async move {
        let mut stream = api::stream::available_games_events(user_id()).await?;
        while let Some(Ok(event)) = stream.next().await {
            Toast::available_game(event);
        }
        dioxus::Ok(())
    });

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
