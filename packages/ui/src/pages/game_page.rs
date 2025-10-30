use dioxus::prelude::*;
use dto::{CardDTO, CutForDealStateDTO, GameIdDTO, Phase, Player, UserGameDTO, UserIdDTO};

use crate::components::CardView;
use crate::route::Route;

#[component]
pub fn GamePage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = provide_context(game_id);

    let mut game = use_signal(|| None::<UserGameDTO>);
    provide_context(game);

    // let mut game_stream = use_action(move || async move {
    //     let mut stream = api::user_game_stream(*user_id.read(), game_id).await?;
    //     while let Some(Ok(updated_game)) = stream.next().await {
    //         game.set(Some(updated_game));
    //     }
    //     dioxus::Ok(())
    // });

    let _initial_game = use_resource(move || async move {
        let initial_game = api::get_game(*user_id.read(), game_id).await?;
        game.set(initial_game);
        // game_stream.call();
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
    debug!("ActiveGame: {game:?}");
    match &game.phase() {
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
        Phase::Active { dealer, crib, .. } => {
            rsx! { p { "Decided: {dealer:?} {crib:?}" } }
            // rsx! { InProgress { user_state, opponent_state, crib, cut, plays, winner }}
        }
    }
}

#[component]
fn Starting(
    user_cut: Option<CardDTO>,
    opponent_cut: Option<CardDTO>,
    dealer: Option<Player>,
) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let navigator = use_navigator();

    let mut user_cut = use_signal(|| user_cut);
    let mut opponent_cut = use_signal(|| opponent_cut);
    let mut dealer = use_signal(|| dealer);

    let on_cut_for_deal = move |_| {
        spawn(async move {
            match api::cut_for_deal(*user_id.read(), game_id).await {
                Ok(_state) => {
                    // cut_for_deal_state.set(state);
                }
                Err(error) => {
                    let error = error.to_string();
                    navigator.push(Route::OopsPage { error });
                }
            }
        });
    };

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

    let _ = use_resource(move || async move {
        let mut stream = api::user_game_stream(*user_id.read(), game_id).await?;
        while let Some(Ok(updated_game)) = stream.next().await {
            debug!("GamePage::Starting: {updated_game:?}");
            user_cut.set(updated_game.user_cut().cloned());
            opponent_cut.set(updated_game.opponent_cut().cloned());
            dealer.set(updated_game.dealer().cloned());
        }
        dioxus::Ok(())
    });

    let mut user_has_cut = use_signal(|| false);
    let mut opponent_has_cut = use_signal(|| false);
    let mut dealer_selected = use_signal(|| false);

    use_effect(move || {
        debug!(
            "GamePage::Starting:use_effect: {} {} {}",
            user_cut.read().is_some(),
            opponent_cut.read().is_some(),
            dealer.read().is_some()
        );
        user_has_cut.set(user_cut.read().is_some());
        opponent_has_cut.set(opponent_cut.read().is_some());
        dealer_selected.set(dealer.read().is_some());
    });

    rsx! {
        div {
            class: "game-page",
            div {
                class: "starting",
                CardView { card: user_cut() }
                CardView { card: opponent_cut() }
            }
            if let Some(dealer) = dealer() {
                p { "{dealer}" }
            } else {
                p { "No dealer" }
            }
            {
                debug!("GamePage::Starting:matching");
                match (*user_has_cut.read(), *opponent_has_cut.read(), *dealer_selected.read()) {
                    (false, _, _) => rsx! {
                        button {
                            onclick: on_cut_for_deal,
                            "Cut"
                        }
                    },
                    (true, false, _) => rsx! { "Waiting for opponent..." },
                    (true, true, false) => rsx! { "Redraw" },
                    (true, true, true) => rsx! { "Start" },
                }
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
