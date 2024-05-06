use super::card::Card;

use crate::view::PlayerCard;

use leptos::*;

#[component]
pub fn PlayerCard(
    #[prop()]
    card: PlayerCard,
) -> impl IntoView {
    match card {
        PlayerCard::Empty => view! { <p>"Empty"</p> }.into_view(),
        PlayerCard::FaceUp(card) => view! { <Card card=card/> }.into_view(),
    }
}
