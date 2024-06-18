use super::cards::Cards;

use crate::view::prelude::PlayState;

use leptos::*;
use style4rs::style;

#[component]
pub(crate) fn Plays(

    state: PlayState,

) -> impl IntoView {

    let class = style!{
        div {
            flex: 1 1 auto;
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: end;
            gap: 18px;
        }

        .runningtotal {
            flex: 1 1 auto;
            align-self: end;
            font-size: 42px;
        }
    };

    let running_total = state.running_total();
    let current_plays = state.current_plays().into_iter().map(|p| p.card()).collect::<Vec<_>>();
    let previous_plays = state.previous_plays().into_iter().map(|p| p.card()).collect::<Vec<_>>();

    view!{
        class = class,
            <div>
                <div>
                    {
                        (!previous_plays.is_empty()).then_some(view! { <Cards cards={previous_plays} opacity="0.1".into() /> }.into_view())
                    }
                    {
                        (!current_plays.is_empty()).then_some(view! { <Cards cards={current_plays} /> }.into_view())
                    }
                </div>
                <div class="runningtotal">
                    { (running_total != 0).then_some( { view! { <p>{running_total}</p> } } ) }
                </div>
            </div>
    }
}
