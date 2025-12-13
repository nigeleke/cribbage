use api::dto::{PeggingBreakdownDTO, UserGameDTO};
use dioxus::prelude::*;

use crate::components::domain::MiniIdCard;

#[component]
pub fn ScoringDetail() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();
    let pegging = use_memo(move || game().pegging);
    let breakdown = use_memo(move || pegging().breakdown);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/scoring_detail.css")},
        div {
            class: "scoring-detail",
            if breakdown().is_empty() {
                Nineteen {}
            } else {
                Breakdown { breakdown }
            }
        }
    }
}

#[component]
fn Breakdown(breakdown: ReadSignal<PeggingBreakdownDTO>) -> Element {
    rsx! {
        for (kind, summary) in breakdown() {
            div { "{kind}" }
            div {
                class: "scoring-detail__breakdowns",
                for cards in summary.breakdown {
                    div {
                        class: "scoring-detail__breakdown",
                        for cid in cards {
                            MiniIdCard { cid }
                        }
                    }
                }
            }
            div { "{summary.points}" }
        }
    }
}

#[component]
fn Nineteen() -> Element {
    rsx! {
        div { }
        img {
            class: "scoring-detail__nineteen",
            src: asset!("/assets/19.png")
        }
        div { }
    }
}
