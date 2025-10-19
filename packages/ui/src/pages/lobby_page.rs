use dioxus::prelude::*;
use dto::{GameIdDTO, UserIdDTO};
// use futures_util::StreamExt;

// use crate::route::Route;

#[component]
pub fn LobbyPage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let mut game = use_signal(|| None);

    dioxus::logger::tracing::info!("LobbyPage 1");

    let mut game_stream = use_action(move || async move {
        dioxus::logger::tracing::info!("LobbyPage 2");
        let mut stream = api::user_game_stream(*user_id.read(), game_id).await?;
        dioxus::logger::tracing::info!("LobbyPage 3");

        while let Some(Ok(updated_game)) = stream.next().await {
            dioxus::logger::tracing::info!("LobbyPage 4");
            game.set(Some(updated_game));
        }

        dioxus::logger::tracing::info!("LobbyPage 5");
        dioxus::Ok(())
    });

    use_effect(move || {
        game_stream.call();
    });

    let navigator = use_navigator();

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
