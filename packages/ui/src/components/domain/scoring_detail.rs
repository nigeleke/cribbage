use api::dto::{PeggingDTO, UserGameDTO};
use dioxus::prelude::*;

use crate::components::domain::MiniCard;

#[component]
pub fn ScoringDetail() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();
    let pegging = use_memo(move || game().pegging);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/scoring_detail.css")},
        div {
            class: "scoring-detail",
            if pegging().is_empty() {
                Nineteen {}
            } else {
                Breakdown { pegging }
            }
        }

    }
}

#[component]
fn Breakdown(pegging: ReadSignal<PeggingDTO>) -> Element {
    rsx! {
        for (kind, summary) in pegging() {
            div { "{kind}" }
            div {
                class: "scoring-detail__breakdowns",
                for cards in summary.breakdown {
                    div {
                        class: "scoring-detail__breakdown",
                        for card in cards {
                            MiniCard { card }
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
        div { "19" }
        div { }
    }
}
