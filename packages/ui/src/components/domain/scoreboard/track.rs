use super::block::Block;

use api::{PlayerDTO, ScoreDTO};
use dioxus::prelude::*;

/// Show a graphical scoring track in the scoreboard.
#[component]
pub fn Track(
    x_offset: usize,
    y_offset: usize,
    player: PlayerDTO,
    score: ReadSignal<ScoreDTO>,
) -> Element {
    let translate = format!("translate({},{})", x_offset, y_offset);

    rsx! {
        g { transform: "{translate}",
            {(0..6).map(|n| {
                let up_base = n*5+1;
                let up_range = up_base..(up_base+5);
                let down_base = 5*n+31;
                let down_range = down_base..(down_base+5);
                rsx! { Block { x_offset: 0, y_offset: {n*42}, up_range, down_range, player, score } }
            })}
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    // use crate::test::LeptosRuntime;
    // use crate::view::Peggings;

    #[test]
    fn track_should_render_6_blocks() {
        // LeptosRuntime::new(
        //     || {
        //         let _ = provide_context(Role::CurrentPlayer);
        //         let _ = provide_context(Peggings::default());

        //         Track(TrackProps {
        //             x_offset: 0,
        //             y_offset: 0,
        //             role: Role::CurrentPlayer,
        //         })
        //     },
        //     |_: &View| {},
        //     |rendered: String| {
        //         let block_count = rendered.matches("leptos-block-start").count();
        //         assert_eq!(block_count, 6);
        //     },
        // )
        // .run()
    }
}
