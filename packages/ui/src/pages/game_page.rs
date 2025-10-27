use dioxus::prelude::*;
use dto::{CardDTO, GameIdDTO, Phase, UserGameDTO, UserIdDTO};

use crate::components::CardView;

#[component]
pub fn GamePage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

    let mut game = use_signal(|| None);
    provide_context(game);

    let _initial_game = use_resource(move || async move {
        let initial_game = api::get_game(*user_id.read(), game_id).await?;
        game.set(initial_game);
        dioxus::Ok(())
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/game_page.css") }
        if let Some(game) = game() {
            ActiveGame { game }
        } else {
            div {
                class: "game-page",
                "Loading..."
            }
        }
    }
}

#[component]
fn ActiveGame(game: UserGameDTO) -> Element {
    match game.phase() {
        Phase::Lobby => rsx! { Starting { user_cut: None, opponent_cut: None } },
        Phase::CutForDeal {
            user_cut,
            opponent_cut,
        } => {
            let user_cut = user_cut.clone();
            let opponent_cut = opponent_cut.clone();
            rsx! { Starting { user_cut, opponent_cut } }
        }
        Phase::Active { dealer, crib } => {
            rsx! { p { "Decided" } }
            // rsx! { InProgress { user_state, opponent_state, crib, cut, plays, winner }}
        }
    }
}

#[component]
fn Starting(user_cut: Option<CardDTO>, opponent_cut: Option<CardDTO>) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    // let game_id = use_context::<GameId>();

    let mut waiting = use_signal(|| false);

    //     let on_start = move |_| {
    //         spawn(async move {
    //             match start(game_id, user_id()).await {
    //                 Ok(ready) => {
    //                     if !ready {
    //                         waiting.set(true);
    //                     }
    //                 }
    //                 Err(e) => panic!("start game failed: {}", e.to_string()),
    //             }
    //         });
    //     };

    //     let on_redraw = move |_| {
    //         spawn(async move {
    //             match redraw(game_id, user_id()).await {
    //                 Ok(ready) => {
    //                     if !ready {
    //                         waiting.set(true);
    //                     }
    //                 }
    //                 Err(e) => panic!("redraw game failed: {}", e.to_string()),
    //             }
    //         });
    //     };

    rsx! {
        div {
            class: "game-page",
            div {
                class: "starting",
                CardView { card: user_cut }
                CardView { card: opponent_cut }
            }
            // if let Some(dealer) = dealer {
            //     if dealer == Role::User {
            //         h2 { "You deal" }
            //     } else {
            //         h2 { "Opponent deals" }
            //     }
            //     button {
            //        onclick: on_start,
            //        "Ok"
            //     }
            // } else {
            //     button {
            //        onclick: on_redraw,
            //        "Redraw"
            //     }
            // }
        }
    }
}

// #[component]
// fn InProgress(
//     user_state: PlayerState,
//     opponent_state: PlayerState,
//     crib: Vec<CardState>,
//     cut: Option<Card>,
//     plays: Option<Plays>,
//     winner: Option<Role>,
// ) -> Element {
//     rsx! {
//         div {
//             class: "in-progress",
//             div {
//                 class: "scoreboard",
//                 h2 { class: "scoreboard-title", "Scoreboard" }
//             }
//             div {
//                 class: "card-container",
//                 h3 { class: "section-title", "Your Hand" }
//             }
//             div {
//                 class: "middle-section",
//                 p { "middle" }
//             }
//             div {
//                 class: "card-container",
//                 h3 { class: "section-title", "Opponent Hand" }
//             }
//             div {
//                 class: "crib-cut-container",
//                 h3 { class: "section-title", "Crib / cut" }
//             }
//         }
//     }
// }
