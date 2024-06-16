use crate::components::cuts::Cuts;
use crate::view::prelude::{Cuts, Role};

use leptos::*;

#[component]
pub(crate) fn Starting(

    cuts: Cuts

) -> impl IntoView {

    view!{
        <Cuts player_cut={cuts[&Role::CurrentPlayer]} opponent_cut={cuts[&Role::Opponent]} />
    }

}
