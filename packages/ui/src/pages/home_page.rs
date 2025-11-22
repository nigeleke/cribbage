// use api::AvailableGamesStreamEvent;
use api::{AvailableGameDTO, AvailableGameEventDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::components::{
    DebouncedInput, HostGameAction, PlayComputerAction, SelectGameAction, Toast,
};
use crate::route::Route;

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
    let mut games = use_signal(Vec::<AvailableGameDTO>::default);

    let user_id = use_context::<Signal<UserIdDTO>>();
    let mut toasts = use_signal(Vec::<String>::new);
    let mut filter = use_signal(String::default);

    let mut has_more = use_signal(|| false);
    let mut since = use_signal(|| api::view::Since::default());

    let fetch_games = {
        move |since2: api::view::Since, replace: bool| async move {
            let result =
                api::view::get_available_games(*user_id.read(), Some(filter()), since2).await;
            match result {
                Ok(response) => {
                    if replace {
                        games.set(Vec::from(response.games()));
                    } else {
                        let mut new_games = response
                            .games()
                            .iter()
                            .filter_map(|g| (!games.read().contains(g)).then_some(g.clone()))
                            .collect::<Vec<_>>();
                        games.write().append(&mut new_games);
                    }
                    since.set(response.since().clone());
                    has_more.set(response.has_more());
                }
                Err(e) => {
                    error!("Failed to fetch games: '{e}'");
                }
            }
        }
    };

    let _ = use_resource(move || async move {
        fetch_games(api::view::Since::default(), true).await;
    });

    let _ = use_resource(move || async move {
        let mut stream = api::stream::available_games_events(*user_id.read()).await?;
        while let Some(Ok(event)) = stream.next().await {
            match event {
                AvailableGameEventDTO::Created { name, .. } => {
                    has_more.set(true);
                    toasts.write().push(format!("{} added", name))
                }
                AvailableGameEventDTO::Removed { game_id, name } => {
                    games.write().retain(|g| g.id() != &game_id);
                    toasts.write().push(format!("{} removed", name))
                }
            }
        }
        dioxus::Ok(())
    });

    rsx! {
        section {
            class: "home-page__join-game-section",
            h2 { "Join a Game" }
            DebouncedInput {
                placeholder: "🔍 Search games...",
                value: filter,
                on_debounced_input: move |value| {
                    filter.set(value);
                    let state = api::view::Since::default();
                    async move {
                        fetch_games(state, true).await;
                    }
                }
            }
            GameList { games }
            button {
                class: "home-page__join-game__more-button",
                disabled: !has_more(),
                onclick: move |_| {
                    async move {
                        fetch_games(since(), false).await;
                    }
                },
                "More..."
            }
            Toast { toasts }
        }
    }
}

#[component]
fn GameList(games: ReadSignal<Vec<AvailableGameDTO>>) -> Element {
    rsx! {
        div {
            class: "home-page__join-game__list",
            ul {
                class: "home-page__join-game__list-items",
                for game in games().into_iter() {
                    li {
                        class: "home-page__join-game__list-item",
                        class: if matches!(&game, AvailableGameDTO::Active { .. }) { "active" },
                        title: "{game.name()}",
                        key: "{game.id()}",
                        SelectGameAction { game: game.clone() }
                    }
                }
            }
        }
    }
}
