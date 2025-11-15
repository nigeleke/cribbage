use api::{
    CardDTO, GameEventDTO, GameIdDTO, Phase, PlayerDTO, PlayerStateDTO, PlaysDTO, UserGameDTO,
    UserIdDTO,
};
use dioxus::prelude::*;

use crate::components::{CardView, Hand, Scoreboard};
use crate::route::Route;

#[component]
pub fn GamePage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = provide_context(game_id);

    let mut game = use_signal(|| None::<UserGameDTO>);
    provide_context(game);

    let mut game_stream = use_action(move || async move {
        let mut stream = api::stream::user_game_stream(*user_id.read(), game_id).await?;
        while let Some(Ok(updated_game)) = stream.next().await {
            game.set(Some(updated_game));
        }
        dioxus::Ok(())
    });

    let _ = use_resource(move || async move {
        let current_game = api::view::get_game(*user_id.read(), game_id).await?;
        game.set(Some(current_game));
        game_stream.call();
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
        Phase::Active { dealer, crib } => {
            let user_state = game().user_state;
            let opponent_state = game().opponent_state;
            let cut = game().cut;
            let plays = game().plays;
            let winner = game().winner;
            rsx! { InProgress { user_state, opponent_state, dealer, crib, cut, plays, winner } }
        }
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

    let mut acknowledge_required = use_signal(|| false);

    let on_acknowledge = move |_| {
        spawn(async move {
            match api::action::acknowledge_cut_for_deal(*user_id.read(), game_id).await {
                Ok(_) => acknowledge_required.set(false),
                Err(error) => {
                    warn!("GamePage:acknowledge:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            }
        });
    };

    let _ = use_resource(move || async move {
        let mut stream = api::stream::user_game_events(*user_id.read(), game_id).await?;
        while let Some(Ok(event)) = stream.next().await {
            match event {
                GameEventDTO::CutForDealDecided => acknowledge_required.set(true),
                GameEventDTO::CutForDealTied => acknowledge_required.set(true),
                _ => {}
            }
        }
        dioxus::Ok(())
    });

    let waiting_for_opponent = rsx! { p { "Waiting for opponent"} };

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
                (Some(_), None, _) => waiting_for_opponent,
                (Some(_), Some(_), None) if acknowledge_required() => redraw_button,
                (Some(_), Some(_), None) => waiting_for_opponent,
                (Some(_), Some(_), Some(dealer)) if acknowledge_required() => start_button,
                (Some(_), Some(_), Some(_)) => waiting_for_opponent,
            }
        }
    }
}

#[component]
fn InProgress(
    user_state: PlayerStateDTO,
    opponent_state: PlayerStateDTO,
    dealer: PlayerDTO,
    crib: Vec<CardDTO>,
    cut: Option<CardDTO>,
    plays: Option<PlaysDTO>,
    winner: Option<PlayerDTO>,
) -> Element {
    let user_score = user_state.score;
    let user_hand = user_state.hand;
    let opponent_score = opponent_state.score;
    let opponent_hand = opponent_state.hand;

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
                    Hand { cards: user_hand }
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
                    h3 { class: "section-title", "Crib / cut" }
                }
            }
        }
    }
}
