use super::card::Card;

use crate::view::OpponentCard;

use leptos::*;

#[component]
pub fn OpponentCard(
    #[prop()]
    card: OpponentCard,
) -> impl IntoView {
    match card {
        OpponentCard::Empty => view! { <p>"Empty"</p> }.into_view(),
        OpponentCard::FaceUp(card) => view! { <Card card=card/> }.into_view(),
        OpponentCard::FaceDown => view! { <Card /> }.into_view(),
    }
}