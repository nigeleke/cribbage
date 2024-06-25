use crate::types::HasPoints;
use crate::view::{Role, Pegging, Peggings};

use leptos::*;

/// Show a single hole in the scoreboard.
#[component]
pub fn Hole(

    x_offset: usize,
    y_offset: usize,
    representation: usize

) -> impl IntoView {
    
    let role = use_context::<Role>().unwrap();
    let scores = use_context::<Peggings>().unwrap();
    let default_score = Pegging::default();
    let score = scores.get(&role).unwrap_or(&default_score);

    let colour = (if role == Role::CurrentPlayer { "lime" } else { "red" }).to_string();
    let fill = match representation {
        0 => colour,
        n if score.front_peg().points() % 60.into() == n.into() => colour,
        n if score.back_peg().points() % 60.into() == n.into() => colour,
        n if n >= 121 => colour,
        _ => "gray".into(),
    };

    let translate = format!("translate({},{})", x_offset, y_offset);

    view! {
        <g transform=translate>
            <circle cx="2" cy="2" r="2" fill=fill />
        </g>
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::LeptosRuntime;

    #[test]
    fn hole_should_render_unoccupied() {
        LeptosRuntime::new(
            || {
                let _ = provide_context(Role::CurrentPlayer);
                let _ = provide_context(Peggings::default());
                Hole(HoleProps { x_offset: 10, y_offset: 20, representation: 30, })
            },
            |_: &View| {},
            |rendered: String| {
                assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
                assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="gray""#));
            }
        ).run()
    }   

    #[test]
    fn hole_should_render_occupied_by_current_player() {
        LeptosRuntime::new(
            || {
                let _ = provide_context(Role::CurrentPlayer);

                let mut peggings = Peggings::default();
                let pegging = Pegging::default().add(30.into());
                let _ = peggings.insert(Role::CurrentPlayer, pegging);
                let _ = provide_context(peggings);
            
                Hole(HoleProps { x_offset: 10, y_offset: 20, representation: 30, })
            },
            |_: &View| {},
            |rendered: String| {
                assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
                assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="lime""#));
            }
        ).run()
    }   

    #[test]
    fn hole_should_render_occupied_by_opponent() {
        LeptosRuntime::new(
            || {
                let _ = provide_context(Role::Opponent);

                let mut peggings = Peggings::default();
                let pegging = Pegging::default().add(30.into());
                let _ = peggings.insert(Role::Opponent, pegging);
                let _ = provide_context(peggings);
            
                Hole(HoleProps { x_offset: 10, y_offset: 20, representation: 30, })
            },
            |_: &View| {},
            |rendered: String| {
                assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
                assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="red""#));
            }
        ).run()
    }

    #[test]
    fn start_hole_should_render_players_score_zero() {
        LeptosRuntime::new(
            || {
                let _ = provide_context(Role::CurrentPlayer);

                let mut peggings = Peggings::default();
                let pegging = Pegging::default().add(30.into());
                let _ = peggings.insert(Role::Opponent, pegging);
                let _ = provide_context(peggings);
            
                Hole(HoleProps { x_offset: 10, y_offset: 20, representation: 0, })
            },
            |_: &View| {},
            |rendered: String| {
                assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
                assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="lime""#));
            }
        ).run()
    }

    #[test]
    fn winning_hole_should_render_players_eq_121() {
        LeptosRuntime::new(
            || {
                let _ = provide_context(Role::CurrentPlayer);

                let mut peggings = Peggings::default();
                let pegging = Pegging::default().add(121.into());
                let _ = peggings.insert(Role::Opponent, pegging);
                let _ = provide_context(peggings);
            
                Hole(HoleProps { x_offset: 10, y_offset: 20, representation: 121, })
            },
            |_: &View| {},
            |rendered: String| {
                assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
                assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="lime""#));
            }
        ).run()
    }

    #[test]
    fn winning_hole_should_render_players_gt_121() {
        LeptosRuntime::new(
            || {
                let _ = provide_context(Role::CurrentPlayer);

                let mut peggings = Peggings::default();
                let pegging = Pegging::default().add(122.into());
                let _ = peggings.insert(Role::Opponent, pegging);
                let _ = provide_context(peggings);
            
                Hole(HoleProps { x_offset: 10, y_offset: 20, representation: 121, })
            },
            |_: &View| {},
            |rendered: String| {
                assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
                assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="lime""#));
            }
        ).run()
    }
}
