use crate::components::phases::{CuttingForDeal, Discarding, InLobby, Playing};
use api::{GameIdDTO, PhaseDTO, UserGameDTO, UserIdDTO};
use dioxus::prelude::*;

#[component]
pub fn GamePage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = provide_context(game_id);

    let mut game = use_signal(Option::<UserGameDTO>::default);

    let mut game_stream = use_action(move || async move {
        let mut stream = api::stream::user_game_stream(*user_id.read(), game_id).await?;
        while let Some(Ok(updated_game)) = stream.next().await {
            debug!("GamePage: updated for {user_id} {updated_game:?}");
            game.set(Some(updated_game));
        }
        dioxus::Ok(())
    });

    let _ = use_resource(move || async move {
        let current_game = api::view::get_game(*user_id.read(), game_id).await?;
        game_stream.call();
        game.set(Some(current_game));
        dioxus::Ok(())
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/game_page.css") }
        div {
            class: "game-page",
            if let Some(game) = game() {
                ActiveGame { game }
            } else {
                div { "Loading..." }
            }
        }
    }
}

#[component]
fn ActiveGame(game: ReadSignal<UserGameDTO>) -> Element {
    provide_context(game);

    match game().phase {
        PhaseDTO::InLobby => rsx! { InLobby {} },
        PhaseDTO::CuttingForDeal => rsx! { CuttingForDeal {} },
        PhaseDTO::Discarding => rsx! { Discarding {} },
        PhaseDTO::Playing => rsx! { Playing {} },
        PhaseDTO::ScoringPone => rsx! {},
        PhaseDTO::ScoringDealer => rsx! {},
        PhaseDTO::ScoringCrib => rsx! {},
        PhaseDTO::Finished => rsx! {},
    }
}

// #[component]
// fn InProgress() -> Element {
//     let game = use_context::<ReadSignal<UserGameDTO>>();

//     let phase = use_memo(move || game().phase);
//     let user_score = use_memo(move || game().user_state.score);
//     let user_hand = use_memo(move || game().user_state.hand);
//     let opponent_score = use_memo(move || game().opponent_state.score);
//     let opponent_hand = use_memo(move || game().opponent_state.hand);
//     let dealer = use_memo(move || game().dealer.expect("dealer must have been selected"));
//     let crib = use_memo(move || game().crib.cards);
//     let starter_cut = use_memo(move || game().crib.starter_cut);
//     let plays = use_memo(move || game().plays);

//     rsx! {
//         div {
//             class: "game-game__in-progress",
//             div {
//                 class: "scoreboard",
//                 Scoreboard { user_score, opponent_score }
//             }
//             div {
//                 class: "card-container",
//                 match *phase.read() {
//                     PhaseDTO::Discarding => rsx! {
//                         UserHand {
//                             cards: user_hand,
//                             DiscardingControls { }
//                         }
//                     },
//                     PhaseDTO::Playing => rsx! {
//                         UserHand {
//                             cards: user_hand,
//                             PlayingControls { }
//                         }
//                     },
//                     _ => rsx! {"unsupported phase"}
//                 }
//             }
//             div {
//                 class: "middle-section",
//                 Plays { plays }
//             }
//             div {
//                 class: "card-container",
//                 Hand { cards: opponent_hand }
//             }
//             div {
//                 class: "crib-cut-container",
//                 CribAndCut { dealer, cards: crib, starter_cut }
//             }
//         }
//     }
// }
