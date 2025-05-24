use crate::{components::DebouncedInput, route::Route};
use api::{
    UnstartedGame, UnstartedGamesEvent, UnstartedGamesRequest, UnstartedGamesState, UserId,
    activate_game, fetch_unstarted_games, new_computer_game, new_human_game,
    unstarted_games_stream,
};
use dioxus::{logger::tracing::warn, prelude::*};
use futures::stream::StreamExt;

#[component]
pub fn HomePage() -> Element {
    let games = use_signal(Vec::<UnstartedGame>::default);
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
    let user_id = use_context::<Signal<UserId>>();
    let navigator = use_navigator();

    let start_human_game = move |_| {
        spawn(async move {
            match new_human_game(*user_id.read()).await {
                Ok(game) => {
                    navigator.push(Route::LobbyPage { id: *game.id() });
                }
                Err(e) => panic!("start game failed: {}", e.to_string()),
            }
        });
    };

    let start_computer_game = move |_| {
        spawn(async move {
            match new_computer_game(*user_id.read()).await {
                Ok(id) => {
                    navigator.push(Route::GamePage { id });
                }
                Err(e) => panic!("start game failed: {}", e.to_string()),
            };
        });
    };

    rsx! {
        section {
            class: "new-game",
            h2 { "Start a New Game" }
            div {
                class: "new-game-buttons",
                button { onclick: start_human_game, "Play with Friends" }
                button { onclick: start_computer_game, "Play with Computer" }
            }
        }
    }
}

#[component]
fn JoinGameSection() -> Element {
    let user_id = use_context::<Signal<UserId>>();
    let mut games = use_context::<Signal<Vec<UnstartedGame>>>();
    let mut filter = use_signal(String::default);

    let mut has_more = use_signal(|| false);
    let mut next_state = use_signal(UnstartedGamesState::default);

    let fetch_games = {
        move |state: UnstartedGamesState, replace: bool| async move {
            let request = UnstartedGamesRequest::new(user_id(), filter(), state);
            match fetch_unstarted_games(request).await {
                Ok(response) => {
                    if replace {
                        games.set(response.games().clone());
                    } else {
                        games.write().append(&mut response.games().clone());
                    }
                    next_state.set(response.state().clone());
                    has_more.set(response.has_more());
                }
                Err(e) => {
                    panic!("Failed to fetch games: {e}");
                }
            }
        }
    };

    let _ = use_future(move || fetch_games(UnstartedGamesState::default(), true));

    let _ = use_resource(move || async move {
        match unstarted_games_stream().await {
            Ok(stream) => {
                let mut stream = stream.into_inner();
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(event) => match event {
                            UnstartedGamesEvent::NewGame(_) => has_more.set(true),
                            UnstartedGamesEvent::RemovedGame(deleted_game) => {
                                games.write().retain(|game| game != &deleted_game)
                            }
                        },
                        Err(e) => {
                            warn!("Stream error: {:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Failed to fetch stream: {:?}", e);
                return;
            }
        }
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
                    let state = UnstartedGamesState::default();
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
                        fetch_games(next_state(), false).await;
                    }
                },
                "More..."
            }
        }
    }
}

#[component]
fn GameList(games: ReadOnlySignal<Vec<UnstartedGame>>) -> Element {
    let user_id = use_context::<Signal<UserId>>();
    let navigator = use_navigator();

    let create_active_game = move |unstarted_game: UnstartedGame| {
        move |_| {
            let unstarted_game = unstarted_game.clone();
            spawn(async move {
                match activate_game(user_id(), *unstarted_game.id()).await {
                    Ok(id) => navigator.push(Route::GamePage { id }),
                    Err(e) => panic!("Unable to start game: {e}"),
                };
            });
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
                        key: game.id(),
                        onclick: create_active_game(game),
                        span { "{game.to_string()}" }
                    }
                }
            }
        }
    }
}
