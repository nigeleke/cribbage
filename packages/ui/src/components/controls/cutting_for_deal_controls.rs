use crate::components::WaitingForOpponent;
use crate::route::Route;
use api::{GameIdDTO, PendingDTO, UserGameDTO, UserIdDTO};
use dioxus::prelude::*;

#[component]
pub fn CuttingForDealControls() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let game = use_context::<ReadSignal<UserGameDTO>>();

    let user_cut = use_memo(move || game().user_state.cut);
    let opponent_cut = use_memo(move || game().opponent_state.cut);
    let dealer = use_memo(move || game().dealer);
    let pending = use_memo(move || game().pending);

    let navigator = use_navigator();

    let on_cut_for_deal = move |_| {
        spawn(async move {
            match api::action::cut_for_deal(*user_id.read(), game_id).await {
                Ok(_) => {}
                Err(error) => {
                    warn!("CuttingForDealControls:error {error:?}");
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

    rsx! {
        match (user_cut(), opponent_cut(), dealer()) {
            (None, _, _) => rsx! {
                button {
                    onclick: on_cut_for_deal,
                    "Cut for deal"
                }
            },
            (_, Some(_), None) if pending() == PendingDTO::User => rsx! {
                button {
                    onclick: on_acknowledge,
                    "Redraw"
                }
            },
            (_, Some(_), Some(_)) if pending() == PendingDTO::User => rsx! {
                button {
                    onclick: on_acknowledge,
                    "Start"
                }
            },
            _ => rsx! {
                WaitingForOpponent { }
            },
        }
    }
}
