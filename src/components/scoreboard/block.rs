use super::hole::Hole;

use leptos::*;

use std::ops::Range;

/// Show a common block of a track in the scoreboard.
#[component]
pub fn Block(

    x_offset: usize,
    y_offset: usize,
    up_range: Range<usize>,
    down_range: Range<usize>

) -> impl IntoView {

    let translate = format!("translate({},{})", x_offset, y_offset);

    let zipped = up_range.zip(down_range.rev()).enumerate();

    view!{
        <g transform=translate>
            <rect width="20" height="44" rx="3" ry="3" fill="goldenrod" />
            <g transform="translate(2,2)">
                <rect width="16" height="40" rx="2" ry="2" fill="palegoldenrod" />
                <g transform="translate(2,2)">
                    {zipped.map(|(i, (up, down))| view!{
                        <Hole x_offset=0 y_offset={8*i} representation={up} />
                        <Hole x_offset=8 y_offset={8*i} representation={down} />
                    }).collect::<Vec<_>>()}
                </g>
            </g>
        </g>
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::LeptosRuntime;
    use crate::view::{Peggings, Role};

    #[test]
    fn block_should_render_within_a_rectangle() {
        LeptosRuntime::new(
            || {
                let _ = provide_context(Role::CurrentPlayer);
                let _ = provide_context(Peggings::default());
        
                Block(BlockProps { x_offset: 0, y_offset: 0, up_range: 0..5, down_range: 5..10 })        
            },
            |_: &View| {},
            |rendered: String| {
                assert!(rendered.contains(r#"<rect width="20" height="44" rx="3" ry="3" fill="goldenrod""#));
                assert!(rendered.contains(r#"<rect width="16" height="40" rx="2" ry="2" fill="palegoldenrod""#));
            }
        ).run()
    }

    #[test]
    fn block_should_render_10_holes() {
        LeptosRuntime::new(
            || {
                let _ = provide_context(Role::CurrentPlayer);
                let _ = provide_context(Peggings::default());
        
                Block(BlockProps { x_offset: 0, y_offset: 0, up_range: 0..5, down_range: 5..10 })        
            },
            |_: &View| {},
            |rendered: String| {
                let hole_count = rendered.matches("<circle ").count();
                assert_eq!(hole_count, 10);
            }
        ).run()
    }
}