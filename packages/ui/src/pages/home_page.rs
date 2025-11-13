// use api::AvailableGamesStreamEvent;
use api::{AvailableGameDTO, GameIdDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::components::{DebouncedInput, Toast};
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
    let user_id = use_context::<Signal<UserIdDTO>>();
    let navigator = use_navigator();

    let host_game = move |_| {
        spawn(async move {
            match api::action::host_game(*user_id.read()).await {
                Ok(game_id) => {
                    navigator.push(Route::LobbyPage { game_id });
                }
                Err(error) => {
                    warn!("HomePage:host_game:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            }
        });
    };

    let play_computer = move |_| {
        spawn(async move {
            match api::action::play_computer(*user_id.read()).await {
                Ok(game_id) => {
                    navigator.push(Route::GamePage { game_id });
                }
                Err(error) => {
                    warn!("HomePage:play_computer:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            };
        });
    };

    rsx! {
        section {
            class: "new-game",
            h2 { "Start a New Game" }
            div {
                class: "new-game-buttons",
                button { onclick: host_game, "Play with Friends" }
                button { disabled: true, onclick: play_computer, "Play with Computer" }
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

    let mut available_game_events = use_resource(move || async move {
        let mut stream = api::stream::available_games_events(*user_id.read()).await?;
        while let Some(Ok(event)) = stream.next().await {
            //     match event {
            //         AvailableGamesStreamEvent::Added(game) => {
            //             has_more.set(true);
            //             toasts.write().push(format!("{} added", game.name()))
            //         }
            //         AvailableGamesStreamEvent::Removed(game) => {
            //             games.write().retain(|g| g.id() != game.id());
            //             toasts.write().push(format!("{} removed", game.name()))
            //         }
            //     }
        }
        dioxus::Ok(())
    });

    rsx! {
        section {
            class: "join-game",
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
                class: "more-button",
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
    let user_id = use_context::<Signal<UserIdDTO>>();
    let navigator = use_navigator();

    let mut join_game = use_action(move |game_id| async move {
        match api::action::join_game(user_id(), game_id).await {
            Ok(_) => navigator.push(Route::GamePage { game_id }),
            Err(error) => {
                warn!("HomePage:join_game:error {error:?}");
                let error = error.to_string();
                navigator.push(Route::ErrorPage { error })
            }
        };

        dioxus::Ok(())
    });

    let select_game = |available_game: AvailableGameDTO| {
        move |_| match available_game {
            AvailableGameDTO::Lobby { game_id, .. } => {
                join_game.call(game_id);
            }
            AvailableGameDTO::Active { game_id, .. } => {
                navigator.push(Route::GamePage { game_id });
            }
        }
    };

    rsx! {
        div {
            class: "games-list",
            ul {
                class: "game-items",
                for game in games().into_iter() {
                    li {
                        class: "game-item",
                        class: if matches!(game, AvailableGameDTO::Active { .. }) { "active" },
                        key: "{game.id()}",
                        onclick: select_game(game),
                        span { "{game.name()}" }
                    }
                }
            }
        }
    }
}
