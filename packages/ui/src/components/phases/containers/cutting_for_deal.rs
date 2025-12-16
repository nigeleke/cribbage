use api::dto::UserGameDTO;
use dioxus::prelude::*;

use crate::components::{Card, CuttingForDealControls};

#[component]
pub fn CuttingForDeal() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();

    let user_cut = use_memo(move || game().user_state.cut);
    let opponent_cut = use_memo(move || game().opponent_state.cut);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/cutting_for_deal.css") }
        div {
            class: "cutting-for-deal",
            div {
                class: "cutting-for-deal__cuts",
                Card { card: user_cut() }
                Card { card: opponent_cut() }
            }
            div {
                class: "cutting-for-deal__controls",
                CuttingForDealControls { }
            }
        }
    }
}
