use api::dto::{PlayerDTO, ScoreDTO};
use dioxus::prelude::*;

/// Show a single hole in the scoreboard.
#[component]
pub fn Hole(
    x_offset: usize,
    y_offset: usize,
    representation: usize,
    player: PlayerDTO,
    score: ReadSignal<ScoreDTO>,
) -> Element {
    let colour = (if player == PlayerDTO::User {
        "lime"
    } else {
        "red"
    })
    .to_string();

    let back_peg = score.read().back_peg;
    let front_peg = score.read().front_peg;

    let show = back_peg == representation
        || back_peg == 60 + representation
        || front_peg == representation
        || front_peg == 60 + representation;

    let fill = if show { colour } else { "gray".into() };

    let translate = format!("translate({},{})", x_offset, y_offset);

    rsx! {
        g { transform: "{translate}",
            circle { cx: "2", cy: "2", r: "2", fill: "{fill}" }
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn hole_should_render_unoccupied() {
        // LeptosRuntime::new(
        //     || {
        //         let _ = provide_context(Role::CurrentPlayer);
        //         let _ = provide_context(Peggings::default());
        //         Hole(HoleProps {
        //             x_offset: 10,
        //             y_offset: 20,
        //             representation: 30,
        //         })
        //     },
        //     |_: &View| {},
        //     |rendered: String| {
        //         assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
        //         assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="gray""#));
        //     },
        // )
        // .run()
    }

    #[test]
    fn hole_should_render_occupied_by_current_player() {
        // LeptosRuntime::new(
        //     || {
        //         let _ = provide_context(Role::CurrentPlayer);

        //         let mut peggings = Peggings::default();
        //         let pegging = Pegging::default().add(30.into());
        //         let _ = peggings.insert(Role::CurrentPlayer, pegging);
        //         let _ = provide_context(peggings);

        //         Hole(HoleProps {
        //             x_offset: 10,
        //             y_offset: 20,
        //             representation: 30,
        //         })
        //     },
        //     |_: &View| {},
        //     |rendered: String| {
        //         assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
        //         assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="lime""#));
        //     },
        // )
        // .run()
    }

    #[test]
    fn hole_should_render_occupied_by_opponent() {
        // LeptosRuntime::new(
        //     || {
        //         let _ = provide_context(Role::Opponent);

        //         let mut peggings = Peggings::default();
        //         let pegging = Pegging::default().add(30.into());
        //         let _ = peggings.insert(Role::Opponent, pegging);
        //         let _ = provide_context(peggings);

        //         Hole(HoleProps {
        //             x_offset: 10,
        //             y_offset: 20,
        //             representation: 30,
        //         })
        //     },
        //     |_: &View| {},
        //     |rendered: String| {
        //         assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
        //         assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="red""#));
        //     },
        // )
        // .run()
    }

    #[test]
    fn start_hole_should_render_players_score_zero() {
        // LeptosRuntime::new(
        //     || {
        //         let _ = provide_context(Role::CurrentPlayer);

        //         let mut peggings = Peggings::default();
        //         let pegging = Pegging::default().add(30.into());
        //         let _ = peggings.insert(Role::Opponent, pegging);
        //         let _ = provide_context(peggings);

        //         Hole(HoleProps {
        //             x_offset: 10,
        //             y_offset: 20,
        //             representation: 0,
        //         })
        //     },
        //     |_: &View| {},
        //     |rendered: String| {
        //         assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
        //         assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="lime""#));
        //     },
        // )
        // .run()
    }

    #[test]
    fn winning_hole_should_render_players_eq_121() {
        // LeptosRuntime::new(
        //     || {
        //         let _ = provide_context(Role::CurrentPlayer);

        //         let mut peggings = Peggings::default();
        //         let pegging = Pegging::default().add(121.into());
        //         let _ = peggings.insert(Role::Opponent, pegging);
        //         let _ = provide_context(peggings);

        //         Hole(HoleProps {
        //             x_offset: 10,
        //             y_offset: 20,
        //             representation: 121,
        //         })
        //     },
        //     |_: &View| {},
        //     |rendered: String| {
        //         assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
        //         assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="lime""#));
        //     },
        // )
        // .run()
    }

    #[test]
    fn winning_hole_should_render_players_gt_121() {
        // LeptosRuntime::new(
        //     || {
        //         let _ = provide_context(Role::CurrentPlayer);

        //         let mut peggings = Peggings::default();
        //         let pegging = Pegging::default().add(122.into());
        //         let _ = peggings.insert(Role::Opponent, pegging);
        //         let _ = provide_context(peggings);

        //         Hole(HoleProps {
        //             x_offset: 10,
        //             y_offset: 20,
        //             representation: 121,
        //         })
        //     },
        //     |_: &View| {},
        //     |rendered: String| {
        //         assert!(rendered.contains(r#"<g transform="translate(10,20)"#));
        //         assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="lime""#));
        //     },
        // )
        // .run()
    }
}
