use api::{AvailableGamesRequest, AvailableGamesState, get_available_games, host_game};
use dioxus::prelude::*;
use dto::{AvailableGameDTO, UserIdDTO};

// use futures_util::StreamExt;
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
            match host_game(*user_id.read()).await {
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
    let mut next_state = use_signal(AvailableGamesState::default);

    let fetch_games = {
        move |state: AvailableGamesState, replace: bool| async move {
            let request = AvailableGamesRequest::new(*user_id.read(), filter(), state);
            dioxus::logger::tracing::info!("{request:?}");
            let result = get_available_games(request).await;
            dioxus::logger::tracing::info!("{result:?}");
            // match get_available_games(request).await {
            //     Ok(response) => {
            //         if replace {
            //             games.set(Vec::from(response.games()));
            //         } else {
            //             let mut new_games = response
            //                 .games()
            //                 .iter()
            //                 .filter_map(|g| (!games.read().contains(g)).then_some(g.clone()))
            //                 .collect::<Vec<_>>();
            //             games.write().append(&mut new_games);
            //         }
            //         next_state.set(response.state().clone());
            //         has_more.set(response.has_more());
            //     }
            //     Err(e) => {
            //         panic!("Failed to fetch games: {e}");
            //     }
            // }
        }
    };

    let _ = use_resource(move || async move {
        fetch_games(AvailableGamesState::default(), true).await;
    });

    // use_coroutine(move |_: UnboundedReceiver<()>| async move {
    //     match app_event_stream().await {
    //         Ok(stream) => {
    //             println!("HomePage:: Recevied app_event");
    //             let mut stream = stream.into_inner();
    //             while let Some(event) = stream.next().await {
    //                 println!("HomePage:: Recevied app_event: event: {:?}", event);
    //                 match event {
    //                     Ok(event) => match event {
    //                         AppEvent::NewLobbyGame(_) => has_more.set(true),
    //                         AppEvent::RemovedLobbyGame(deleted_game) => games
    //                             .write()
    //                             .retain(|game| game.id().value() != deleted_game.id().value()),
    //                     },
    //                     Err(e) => {
    //                         warn!("Stream error: {:?}", e);
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             warn!("Failed to fetch stream: {:?}", e);
    //             return;
    //         }
    //     }
    // });

    // use_coroutine(move |_: UnboundedReceiver<()>| async move {
    //     match user_event_stream(*user_id.read()).await {
    //         Ok(stream) => {
    //             println!("HomePage:: Recevied user_event");
    //             let mut stream = stream.into_inner();
    //             while let Some(event) = stream.next().await {
    //                 println!("HomePage:: Recevied user_event: event: {:?}", event);
    //                 match event {
    //                     Ok(event) => match event {
    //                         UserEvent::NewActiveGame(new_game) => {
    //                             let new_game = AvailableGame::from(new_game);
    //                             let game_name = new_game.name().clone();
    //                             games.write().insert(0, new_game);
    //                             toasts.write().push(format!("Someone joined {}", game_name));
    //                         }
    //                         UserEvent::RemovedActiveGame(deleted_game) => games
    //                             .write()
    //                             .retain(|game| game.id().value() != deleted_game.id().value()),
    //                     },
    //                     Err(e) => {
    //                         warn!("Stream error: {:?}", e);
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             warn!("Failed to fetch stream: {:?}", e);
    //             return;
    //         }
    //     }
    // });

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
                    // async move {
                    //     fetch_games(state, true).await;
                    // }
                }
            }
            GameList { games }
            button {
                class: "more-button",
                disabled: !has_more(),
                onclick: move |_| {
                    // async move {
                    //     fetch_games(next_state(), false).await;
                    // }
                },
                "More..."
            }
            Toast { toasts }
        }
    }
}

#[component]
fn GameList(games: ReadOnlySignal<Vec<AvailableGameDTO>>) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let navigator = use_navigator();

    let set_active_game = move |available_game: AvailableGameDTO| {
        move |_: u32| {
            let available_game = available_game.clone();
            match available_game {
                AvailableGameDTO::Lobby { game_id, name: _ } => {
                    spawn(async move {
                        // match activate_game(user_id(), id).await {
                        //     Ok(id) => navigator.push(Route::GamePage { id }),
                        //     Err(e) => panic!("Unable to start game: {e}"),
                        // };
                    });
                }
                AvailableGameDTO::Active { game_id, name: _ } => {
                    // navigator.push(Route::GamePage { game_id });
                }
            }
        }
    };

    rsx! {
        div {
            class: "games-list",
            ul {
                class: "game-items",
                // for game in games().into_iter() {
                //     li {
                //         class: "game-item",
                //         class: if matches!(game, AvailableGameDTO::Active { game_id: _, name: _ }) { "active" },
                //         key: game.id(),
                //         onclick: set_active_game(game),
                //         span { "{game.name().to_string()}" }
                //     }
                // }
            }
        }
    }
}
