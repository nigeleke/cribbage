use super::cards::Cards;

use crate::view::PlayState;

use leptos::*;
use leptos_meta::*;
use thaw::*;

#[component]
pub fn Plays(

    state: PlayState,

) -> impl IntoView {

    let running_total = state.running_total();
    let current_plays = state.current_plays().into_iter().map(|p| p.card()).collect::<Vec<_>>();
    let previous_plays = state.previous_plays().into_iter().map(|p| p.card()).collect::<Vec<_>>();

    view!{
        <Space class="outer" justify=SpaceJustify::SpaceBetween>
            <Space class="inner">
                {
                    (!previous_plays.is_empty()).then_some(view! { <Cards cards={previous_plays} opacity="0.1".into() /> }.into_view())
                }
                {
                    (!current_plays.is_empty()).then_some(view! { <Cards cards={current_plays} /> }.into_view())
                }
            </Space>
            <div class="plays-runningtotal">
                { (running_total != 0.into()).then_some( { view! { <p>{running_total.to_string()}</p> } } ) }
            </div>
        </Space>
        <Style>
            ".thaw-space.outer {
              flex-grow: 1;
            }
            .thaw-space.inner {
              flex-wrap: 1;
            }
            .plays-runningtotal {
              font-size: 24pt;
            }"
        </Style>
    }
}
