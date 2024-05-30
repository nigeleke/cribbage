use crate::view::prelude::CardSlot;

use super::card::Card;

use leptos::*;
use style4rs::style;

/// The Cards component shows a set of cards (in the order provided).
/// The optional stacked setting will show the cards on top of each other if true.
/// If the on_selected is provided then it will be triggered when any
/// of the card's selected state changes.
#[component]
pub fn Cards(
    #[prop()] cards: Vec<CardSlot>,
    #[prop(optional)] stacked: bool,
    #[prop(optional)] on_selected: Option<WriteSignal<Vec<bool>>>,
) -> impl IntoView {

    let class = style!{
        div {
            display: flex;
            flex-direction: row;
            gap: 24px;
        }
    };
    
    let selections = (0..cards.len()).map(|_| create_rw_signal(false)).collect::<Vec<_>>();
    let wo_selections = selections.iter().map(|s| s.write_only()).collect::<Vec<_>>();

    view!{
        class = class,
        <div>
            {
                create_effect(move |_| {
                    if let Some(on_selected) = on_selected {
                        let selections = selections
                            .iter()
                            .map(|s| s())
                            .collect::<Vec<_>>();
                        on_selected.update(|s| *s = selections);
                    };
                });

                let n = if stacked { 1 } else { cards.len() };
            
                if on_selected.is_none() {
                    cards.iter().take(n).map(move |card| view!{ <Card card={*card} /> }).collect::<Vec<_>>()
                } else {
                    cards.iter().take(n).enumerate().map(move |(i, card)| view!{ <Card card={*card} on_selected={wo_selections[i]} /> }).collect::<Vec<_>>()
                }
            }
        </div>
    }

}
