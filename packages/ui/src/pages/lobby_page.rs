use dioxus::prelude::*;
use dto::{GameIdDTO, Phase, UserGameDTO, UserIdDTO};

use crate::Route;

#[component]
pub fn LobbyPage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

    let navigator = use_navigator();

    let mut game = use_signal(|| None::<UserGameDTO>);

    use_effect(move || {
        info!("LobbyPage::effect");
        if let Some(game) = game() {
            info!("LobbyPage::effect triggered: {game:?}");
            match game.phase() {
                Phase::Lobby => {}
                _ => {
                    info!("LobbyPage::effect navigating to GamePage {game_id}");
                    navigator.replace(Route::GamePage { game_id });
                }
            }
        }
    });

    let mut game_stream = use_action(move || async move {
        info!("LobbyPage::listening-api::user_game_stream");
        let mut stream = api::user_game_stream(*user_id.read(), game_id).await?;

        while let Some(Ok(updated_game)) = stream.next().await {
            info!("LobbyPage::received-api::user_game_stream: {updated_game:?}");
            game.set(Some(updated_game));
        }

        info!("LobbyPage::finished-api::user_game_stream");
        dioxus::Ok(())
    });

    let _initial_game = use_resource(move || async move {
        let initial_game = api::get_game(*user_id.read(), game_id).await?;
        game.set(initial_game);
        game_stream.call();
        dioxus::Ok(())
    });

    // let mut _game_stream = use_action(move || async move {
    //     let mut stream = api::user_game_stream(*user_id.read(), game_id).await?;
    //     // let _ = spawn(async move {
    //     //     loop {
    //     //         match stream.next().await {
    //     //             Some(Ok(updated_game)) => game.set(Some(updated_game)),
    //     //             Some(Err(e)) => {
    //     //                 warn!("user_game_stream: {e}");
    //     //                 break;
    //     //             }
    //     //             None => sleep(std::time::Duration::from_secs(1)).await,
    //     //         }
    //     //     }
    //     // });
    //     dioxus::Ok(())
    // })
    // .call();

    // use_coroutine(move |_: UnboundedReceiver<()>| async move {
    //     match user_event_stream(user_id()).await {
    //         Ok(stream) => {
    //             let mut stream = stream.into_inner();
    //             while let Some(event) = stream.next().await {
    //                 println!("*()*** On lobby page: received event {:?}", event);
    //                 match event {
    //                     Ok(event) => match event {
    //                         UserEvent::NewActiveGame(game) => {
    //                             println!(
    //                                 "*()*** On lobby page: received new active game {:?}",
    //                                 game
    //                             );
    //                             if game.id() == &id {
    //                                 println!("*()*** Matched");
    //                                 navigator.replace(Route::GamePage { id });
    //                             }
    //                         }
    //                         _ => {}
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
        document::Stylesheet { href: asset!("/assets/css/lobby_page.css") }
        if let Some(game) = game() {
            div {
               class: "lobby-page",
               "The game "
               span {
                  class: "game-name",
                  "{game.name()}"
               }
               " is waiting for an opponent"
            }
        } else {
            div {
                class: "lobby-page",
                "Loading..."
            }
        }
    }
}
