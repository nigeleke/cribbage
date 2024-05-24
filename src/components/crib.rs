use crate::view::{CardSlot, Crib};

use super::card::Card;
use super::cards::Cards;

use leptos::*;

#[component]
pub fn Crib(

    #[prop()]
    crib: Crib,

    #[prop(optional)]
    stacked: bool

) -> impl IntoView {

    if crib.is_empty() {
        view! { <Card card=CardSlot::Placeholder /> }.into_view()
    } else {
        view! { <Cards cards=crib stacked=stacked /> }
    }

}
