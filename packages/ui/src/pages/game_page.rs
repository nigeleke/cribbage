use api::{CardDTO, GameEventDTO, GameIdDTO, Phase, PlayerDTO, UserGameDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::components::CardView;
use crate::route::Route;

#[component]
pub fn GamePage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = provide_context(game_id);

    let mut game = use_signal(|| None::<UserGameDTO>);
    provide_context(game);

    let mut game_stream = use_action(move || async move {
        debug!(">>> GamePage::game_stream 0");
        let mut stream = api::stream::user_game_stream(*user_id.read(), game_id).await?;
        debug!(">>> GamePage::game_stream 1");
        while let Some(Ok(updated_game)) = stream.next().await {
            debug!("***** Setting {user_id} game as {updated_game:?}");
            game.set(Some(updated_game));
        }
        dioxus::Ok(())
    });

    let _ = use_resource(move || async move {
        let game0 = api::view::get_game(*user_id.read(), game_id).await?;
        game.set(Some(game0));
        game_stream.call();
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
fn ActiveGame(game: ReadSignal<UserGameDTO>) -> Element {
    debug!("ActiveGame: {game:?}");
    match game().phase() {
        Phase::Lobby => rsx! { Starting { user_cut: None, opponent_cut: None } },
        Phase::CutForDeal {
            user_cut,
            opponent_cut,
            dealer,
        } => {
            let user_cut = user_cut.clone();
            let opponent_cut = opponent_cut.clone();
            let dealer = dealer.clone();
            rsx! { Starting { user_cut, opponent_cut, dealer } }
        }
        Phase::Active { .. } => rsx! { p{"Active Phase"} },
        // Phase::Active { dealer, crib, .. } => {
        //     rsx! { p { "Decided: {dealer:?} {crib:?}" } }
        //     rsx! { InProgress { user_state, opponent_state, crib, cut, plays, winner }}
        // }
    }
}

#[component]
fn Starting(
    user_cut: ReadSignal<Option<CardDTO>>,
    opponent_cut: ReadSignal<Option<CardDTO>>,
    dealer: ReadSignal<Option<PlayerDTO>>,
) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let mut can_start = use_signal(|| false);
    let mut can_redraw = use_signal(|| false);

    let _ = use_resource(move || async move {
        let mut stream = api::stream::user_game_events(*user_id.read(), game_id).await?;
        while let Some(Ok(event)) = stream.next().await {
            debug!("GamePage::user_game_events: {user_id} {event:?}");
            match event {
                GameEventDTO::CutForDealDecided => can_start.set(true),
                GameEventDTO::CutForDealTied => can_redraw.set(true),
                _ => {}
            }
        }
        debug!("GamePage::user_game_events: {user_id} --done");
        dioxus::Ok(())
    });

    let navigator = use_navigator();

    let on_cut_for_deal = move |_| {
        spawn(async move {
            match api::action::cut_for_deal(*user_id.read(), game_id).await {
                Ok(_state) => {
                    // cut_for_deal_state.set(state);
                }
                Err(error) => {
                    warn!("GamePage:cut_for_deal:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            }
        });
    };

    // let on_acknowledge_cut_for_deal = move |_| {};

    //     //     let on_start = move |_| {
    //     //         spawn(async move {
    //     //             match start(game_id, user_id()).await {
    //     //                 Ok(ready) => {
    //     //                     if !ready {
    //     //                         waiting.set(true);
    //     //                     }
    //     //                 }
    //     //                 Err(e) => panic!("start game failed: {}", e.to_string()),
    //     //             }
    //     //         });
    //     //     };

    //     //     let on_redraw = move |_| {
    //     //         spawn(async move {
    //     //             match redraw(game_id, user_id()).await {
    //     //                 Ok(ready) => {
    //     //                     if !ready {
    //     //                         waiting.set(true);
    //     //                     }
    //     //                 }
    //     //                 Err(e) => panic!("redraw game failed: {}", e.to_string()),
    //     //             }
    //     //         });
    //     //     };

    //     let _ = use_resource(move || async move {
    //         let mut stream = api::user_game_stream(*user_id.read(), game_id).await?;
    //         while let Some(Ok(updated_game)) = stream.next().await {
    //             debug!("GamePage::Starting: {updated_game:?}");
    //             user_cut.set(updated_game.user_cut().cloned());
    //             opponent_cut.set(updated_game.opponent_cut().cloned());
    //             dealer.set(updated_game.dealer().cloned());
    //         }
    //         dioxus::Ok(())
    //     });

    //     let mut user_has_cut = use_signal(|| false);
    //     let mut opponent_has_cut = use_signal(|| false);
    //     let mut dealer_selected = use_signal(|| false);

    //     use_effect(move || {
    //         debug!(
    //             "GamePage::Starting:use_effect: {} {} {}",
    //             user_cut.read().is_some(),
    //             opponent_cut.read().is_some(),
    //             dealer.read().is_some()
    //         );
    //         user_has_cut.set(user_cut.read().is_some());
    //         opponent_has_cut.set(opponent_cut.read().is_some());
    //         dealer_selected.set(dealer.read().is_some());
    //     });

    rsx! {
        div {
            class: "game-page",
            div {
                class: "starting",
                CardView { card: user_cut() }
                CardView { card: opponent_cut() }
            }
            match (user_cut(), opponent_cut(), dealer()) {
                (None, _, _) => rsx! {
                    button {
                        onclick: on_cut_for_deal,
                        "Cut for deal"
                    }
                },
                (Some(_), None, _) => rsx! { p { "Waiting for opponent"} },
                (Some(_), Some(_), None) => rsx! { p { "No dealer declared - Acknowledge??" } },
                (Some(_), Some(_), Some(dealer)) => rsx! { p { "Dealer declared {dealer}" } },
            }

            if can_start() {
                button { "Start" }
            }

            if can_redraw() {
                button { "Redraw" }
            }
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
