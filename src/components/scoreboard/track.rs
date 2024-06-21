use super::block::Block;

use crate::view::Role;

use leptos::*;

/// Show a graphical scoring track in the scoreboard.
#[component]
pub fn Track(

    x_offset: usize,
    y_offset: usize,
    role: Role

) -> impl IntoView {

    provide_context(role);

    let translate = format!("translate({},{})", x_offset, y_offset);

    view! {
        <g transform=translate>
            {move || (0..6).map(|n| { 
                let up_base = n*5+1;
                let up_range = up_base..(up_base+5);
                let down_base = 5*n+31;
                let down_range = down_base..(down_base+5);
                view!{ <Block x_offset=0 y_offset={n*42} up_range=up_range down_range=down_range /> }
            }).collect::<Vec<_>>()}
        </g>
    }
}

