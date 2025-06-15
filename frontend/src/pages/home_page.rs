use crate::{
    components::{DebouncedInput, Toast},
    route::Route,
};
use api::{
    AppEvent, AvailableGame, AvailableGamesRequest, AvailableGamesState, UserEvent, UserId,
    activate_game, app_event_stream, fetch_available_games, new_computer_game, new_human_game,
    user_event_stream,
};
use dioxus::{logger::tracing::warn, prelude::*};
use futures_util::StreamExt;

#[component]
pub fn HomePage() -> Element {
    let games = use_signal(Vec::<AvailableGame>::default);
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
                    println!("Navigating to LobbyPage with id: {}", game.id());
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
                    println!("Navigating to GamePage with id: {}", id);
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
                button { disabled: true, onclick: start_computer_game, "Play with Computer" }
            }
        }
    }
}

#[component]
fn JoinGameSection() -> Element {
    let user_id = use_context::<Signal<UserId>>();
    let mut games = use_context::<Signal<Vec<AvailableGame>>>();
    let mut toasts = use_signal(Vec::<String>::new);
    let mut filter = use_signal(String::default);

    let mut has_more = use_signal(|| false);
    let mut next_state = use_signal(AvailableGamesState::default);

    let fetch_games = {
        move |state: AvailableGamesState, replace: bool| async move {
            let request = AvailableGamesRequest::new(user_id(), filter(), state);
            match fetch_available_games(request).await {
                Ok(response) => {
                    if replace {
                        games.set(response.games().clone());
                    } else {
                        let mut new_games = response
                            .games()
                            .iter()
                            .filter_map(|g| (!games.read().contains(g)).then_some(g.clone()))
                            .collect::<Vec<_>>();
                        games.write().append(&mut new_games);
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

    let _ = use_resource(move || async move {
        fetch_games(AvailableGamesState::default(), true).await;
    });

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        match app_event_stream().await {
            Ok(stream) => {
                println!("HomePage:: Recevied app_event");
                let mut stream = stream.into_inner();
                while let Some(event) = stream.next().await {
                    println!("HomePage:: Recevied app_event: event: {:?}", event);
                    match event {
                        Ok(event) => match event {
                            AppEvent::NewLobbyGame(_) => has_more.set(true),
                            AppEvent::RemovedLobbyGame(deleted_game) => games
                                .write()
                                .retain(|game| game.id().value() != deleted_game.id().value()),
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

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        match user_event_stream(*user_id.read()).await {
            Ok(stream) => {
                println!("HomePage:: Recevied user_event");
                let mut stream = stream.into_inner();
                while let Some(event) = stream.next().await {
                    println!("HomePage:: Recevied user_event: event: {:?}", event);
                    match event {
                        Ok(event) => match event {
                            UserEvent::NewActiveGame(new_game) => {
                                let new_game = AvailableGame::from(new_game);
                                let game_name = new_game.name().clone();
                                games.write().insert(0, new_game);
                                toasts.write().push(format!("Someone joined {}", game_name));
                            }
                            UserEvent::RemovedActiveGame(deleted_game) => games
                                .write()
                                .retain(|game| game.id().value() != deleted_game.id().value()),
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
                    let state = AvailableGamesState::default();
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
            Toast { toasts }
        }
    }
}

#[component]
fn GameList(games: ReadOnlySignal<Vec<AvailableGame>>) -> Element {
    let user_id = use_context::<Signal<UserId>>();
    let navigator = use_navigator();

    let set_active_game = move |available_game: AvailableGame| {
        move |_| {
            let available_game = available_game.clone();
            match available_game {
                AvailableGame::Lobby { id, name: _ } => {
                    spawn(async move {
                        match activate_game(user_id(), id).await {
                            Ok(id) => navigator.push(Route::GamePage { id }),
                            Err(e) => panic!("Unable to start game: {e}"),
                        };
                    });
                }
                AvailableGame::Active { id, name: _ } => {
                    navigator.push(Route::GamePage { id });
                }
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
                        class: if matches!(game, AvailableGame::Active { id: _, name: _ }) { "active" },
                        key: game.id(),
                        onclick: set_active_game(game),
                        span { "{game.to_string()}" }
                    }
                }
            }
        }
    }
}
