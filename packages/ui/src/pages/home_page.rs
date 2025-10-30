use api::AvailableGamesStreamEvent;
use dioxus::prelude::*;
use dto::{AvailableGameDTO, UserIdDTO};

use crate::components::{DebouncedInput, Toast};
use crate::route::Route;

#[component]
pub fn HomePage() -> Element {
    let games = use_signal(Vec::<AvailableGameDTO>::default);
    provide_context(games);

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
            match api::host_game(*user_id.read()).await {
                Ok(game_id) => {
                    navigator.push(Route::LobbyPage { game_id });
                }
                Err(error) => {
                    let error = error.to_string();
                    navigator.push(Route::OopsPage { error });
                }
            }
        });
    };

    let play_computer = move |_| {
        // spawn(async move {
        //     match new_computer_game(*user_id.read()).await {
        //         Ok(id) => {
        //             println!("Navigating to GamePage with id: {}", id);
        //             navigator.push(Route::GamePage { id });
        //         }
        //         Err(e) => panic!("start game failed: {}", e.to_string()),
        //     };
        // });
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
    let user_id = use_context::<Signal<UserIdDTO>>();
    let mut games = use_context::<Signal<Vec<AvailableGameDTO>>>();
    let mut toasts = use_signal(Vec::<String>::new);
    let mut filter = use_signal(String::default);

    let mut has_more = use_signal(|| false);
    let mut since = use_signal(|| api::Since::default());

    let fetch_games = {
        move |since2: api::Since, replace: bool| async move {
            let result = api::get_available_games(*user_id.read(), Some(filter()), since2).await;
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
        fetch_games(api::Since::default(), true).await;
    });

    let mut available_game_events = use_resource(move || async move {
        let mut stream = api::available_games_stream(*user_id.read()).await?;
        while let Some(Ok(event)) = stream.next().await {
            match event {
                AvailableGamesStreamEvent::Added(game) => {
                    has_more.set(true);
                    toasts.write().push(format!("{} added", game.name()))
                }
                AvailableGamesStreamEvent::Removed(game) => {
                    games.write().retain(|g| g.id() != game.id());
                    toasts.write().push(format!("{} removed", game.name()))
                }
            }
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
                    let state = api::Since::default();
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
        match api::join_game(user_id(), game_id).await {
            Ok(game_id) => navigator.push(Route::GamePage { game_id }),
            Err(error) => {
                let error = error.to_string();
                navigator.push(Route::OopsPage { error })
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
