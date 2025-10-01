use dioxus::logger::tracing::warn;
use dioxus::prelude::*;
use dto::{GameIdDTO, UserIdDTO};
// use futures_util::StreamExt;

// use crate::route::Route;

#[component]
pub fn LobbyPage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    // let mut game = use_signal(|| None);

    // let fetch_game = use_resource(move || async move { fetch_lobby_game(id).await });
    // use_effect(move || {
    //     if let Some(result) = fetch_game() {
    //         game.set(result.ok());
    //     };
    // });

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
        // if let Some(game) = game() {
        //     div {
        //        class: "lobby-page",
        //        "The game "
        //        span {
        //           class: "game-name",
        //           "{game.name()}"
        //        }
        //        " is waiting for an opponent"
        //     }
        // } else {
        //     div {
        //         class: "lobby-page",
        //         "Loading..."
        //     }
        // }
    }
}
