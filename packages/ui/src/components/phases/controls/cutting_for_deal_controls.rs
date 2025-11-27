use api::dto::{GameIdDTO, UserGameDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::{
    Toast,
    components::{Confirmation, WaitingForOpponent, button::Button},
};

#[component]
pub fn CuttingForDealControls() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let game = use_context::<ReadSignal<UserGameDTO>>();

    let user_cut = use_memo(move || game().user_state.cut);
    let opponent_cut = use_memo(move || game().opponent_state.cut);
    let dealer = use_memo(move || game().dealer);

    let mut cut_for_deal_action = use_action(move || async move {
        let result = api::action::cut_for_deal(*user_id.read(), game_id).await;

        match result {
            Ok(_) => (),
            Err(ref error) => {
                warn!("CuttingForDealControls:error {error:?}");
                Toast::command_error("Cut for deal", error.to_string());
            }
        }

        result
    });

    let on_cut_for_deal = move |_| cut_for_deal_action.call();

    let mut acknowledge_action = use_action(move || async move {
        let result = api::action::acknowledge_cut_for_deal(*user_id.read(), game_id).await;

        match result {
            Ok(_) => (),
            Err(ref error) => {
                warn!("GamePage:acknowledge:error {error:?}");
                Toast::command_error("Acknowledge cut for deal", error.to_string());
            }
        }

        result
    });

    let on_acknowledge = move |_| acknowledge_action.call();

    let cuts_and_dealer = (
        user_cut.read().is_some(),
        opponent_cut.read().is_some(),
        dealer.read().is_some(),
    );

    rsx! {
        if matches!(cuts_and_dealer, (false, _, _)) {
            Button {
                onclick: on_cut_for_deal,
                "Cut for deal"
            }
        }
        else if matches!(cuts_and_dealer, (_, true, false)) {
            Confirmation {
                Button {
                    onclick: on_acknowledge,
                    "Redraw"
                }
            }
        } else if matches!(cuts_and_dealer, (_, true, true)) {
            Confirmation {
                Button {
                    onclick: on_acknowledge,
                    "Start"
                }
            }
        } else {
            WaitingForOpponent {}
        }
    }
}
