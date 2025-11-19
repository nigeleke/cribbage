use api::{GameIdDTO, PendingDTO, PhaseDTO, UserGameDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::components::{
    CardView, CribAndCut, DiscardingHand, Hand, PlayingHand, Scoreboard, WaitingForOpponent,
};
use crate::route::Route;

#[component]
pub fn GamePage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = provide_context(game_id);

    let mut game = use_signal(|| None::<UserGameDTO>);

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
        if let Some(game) = game() {
            ActiveGame {
                game
            }
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
    match game().phase {
        PhaseDTO::InLobby => rsx! { Starting { game } },
        PhaseDTO::CuttingForDeal => {
            rsx! { Starting { game } }
        }
        PhaseDTO::Discarding | PhaseDTO::Playing => {
            rsx! { InProgress { game } }
        }
        _ => unimplemented!(),
    }
}

#[component]
fn Starting(game: ReadSignal<UserGameDTO>) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let navigator = use_navigator();

    let on_cut_for_deal = move |_| {
        spawn(async move {
            match api::action::cut_for_deal(*user_id.read(), game_id).await {
                Ok(_) => {}
                Err(error) => {
                    warn!("GamePage:cut_for_deal:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            }
        });
    };

    let on_acknowledge = move |_| {
        spawn(async move {
            match api::action::acknowledge_cut_for_deal(*user_id.read(), game_id).await {
                Ok(_) => {}
                Err(error) => {
                    warn!("GamePage:acknowledge:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            }
        });
    };

    let waiting_for_opponent = rsx! { WaitingForOpponent {  } };

    let cut_for_deal_button = rsx! {
        button {
            onclick: on_cut_for_deal,
            "Cut for deal"
        }
    };

    let redraw_button = rsx! {
        button {
            onclick: on_acknowledge,
            "Redraw"
        }
    };

    let start_button = rsx! {
        button {
            onclick: on_acknowledge,
            "Start"
        }
    };

    let mut user_cut = use_signal(|| game().user_state.cut);
    let mut opponent_cut = use_signal(|| game().opponent_state.cut);
    let mut dealer = use_signal(|| game().dealer);
    let mut pending = use_signal(|| game().pending);

    use_effect(move || {
        let game = game();
        user_cut.set(game.user_state.cut);
        opponent_cut.set(game.opponent_state.cut);
        dealer.set(game.dealer);
        pending.set(game.pending);
    });

    rsx! {
        div {
            class: "game-page",
            div {
                class: "starting",
                CardView { card: user_cut() }
                CardView { card: opponent_cut() }
            }
            match (user_cut(), opponent_cut(), dealer()) {
                (None, _, _) => cut_for_deal_button,
                (_, None, _) => waiting_for_opponent,
                (_, _, None) if pending() == PendingDTO::User => redraw_button,
                (_, _, None) => waiting_for_opponent,
                (_, _, Some(_)) if pending() == PendingDTO::User => start_button,
                (_, _, Some(_)) => waiting_for_opponent,
            }
        }
    }
}

#[component]
fn InProgress(game: ReadSignal<UserGameDTO>) -> Element {
    let phase = game().phase;
    let user_score = game().user_state.score;
    let user_hand = game().user_state.hand;
    let opponent_score = game().opponent_state.score;
    let opponent_hand = game().opponent_state.hand;
    let dealer = game().dealer.expect("dealer must have been selected");
    let crib = game().crib.cards;
    let starter_cut = game().crib.starter_cut;

    rsx! {
        div {
            class: "game-page",
            div {
                class: "in-progress",
                div {
                    class: "scoreboard",
                    Scoreboard { user_score, opponent_score }
                }
                div {
                    class: "card-container",
                    if phase == PhaseDTO::Discarding {
                        DiscardingHand { cards: user_hand }
                    } else if phase == PhaseDTO::Playing {
                        PlayingHand { cards: user_hand }
                    } else {
                        Hand { cards: user_hand }
                    }
                }
                div {
                    class: "middle-section",
                    p { "middle" }
                }
                div {
                    class: "card-container",
                    Hand { cards: opponent_hand }
                }
                div {
                    class: "crib-cut-container",
                    CribAndCut { dealer, cards: crib, starter_cut }
                }
            }
        }
    }
}
