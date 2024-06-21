use crate::view::{CardSlot, Crib};

use super::card::Card;
use super::cards::Cards;

use leptos::*;

/// The Crib component shows the cards in the crib, or a placeholder if the crib is empty.
#[component]
pub fn Crib(

    crib: Crib,

) -> impl IntoView {

    if crib.is_empty() {
        view! { <Card card=CardSlot::Placeholder /> }.into_view()
    } else {
        view! { <Cards cards=crib stacked=true /> }.into_view()
    }

}
