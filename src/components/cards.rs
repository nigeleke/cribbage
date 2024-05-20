use crate::view::CardSlot;

use super::card::Card;

use leptos::*;
use style4rs::style;

#[component]
pub fn Cards(

    #[prop()]
    cards: Vec<CardSlot>,

    #[prop(optional)]
    stacked: bool

) -> impl IntoView {

    let class = style!{
        div {
            display: flex;
            flex-direction: row;
            gap: 24px;
        }
    };

    view!{
        class = class,
        <div>
            {cards.iter().map(|card| view!{ <Card card={card.clone()} /> }).collect::<Vec<_>>()}
        </div>
    }

}
