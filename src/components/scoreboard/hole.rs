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
    use super::HoleProps;

    #[test]
    fn hole_should_render_unoccupied() {
        let runtime = create_runtime();
        let _ = provide_context(Role::CurrentPlayer);
        let _ = provide_context(Peggings::default());

        let hole = Hole(HoleProps { x_offset: 10, y_offset: 20, representation: 30, }).into_view();
        let rendered = hole.render_to_string().to_string();
        runtime.dispose();
        assert!(rendered.contains(r#"<g transform="translate(10,20)"#), "actual {}", rendered);
        assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="gray"#), "actual {}", rendered);
    }   

    #[test]
    fn hole_should_render_occupied_by_current_player() {
        let runtime = create_runtime();
        let _ = provide_context(Role::CurrentPlayer);

        let mut scores = Peggings::default();
        let score = Pegging::default().add(30.into());
        let _ = scores.insert(Role::CurrentPlayer, score);
        let _ = provide_context(scores);

        let hole = Hole(HoleProps { x_offset: 10, y_offset: 20, representation: 30, }).into_view();
        let rendered = hole.render_to_string().to_string();
        runtime.dispose();
        assert!(rendered.contains(r#"<g transform="translate(10,20)"#), "actual {}", rendered);
        assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="lime"#), "actual {}", rendered);
    }   

    #[test]
    fn hole_should_render_occupied_by_opponent() {
        let runtime = create_runtime();
        let _ = provide_context(Role::Opponent);

        let mut scores = Peggings::default();
        let score = Pegging::default().add(30.into());
        let _ = scores.insert(Role::Opponent, score);
        let _ = provide_context(scores);

        let hole = Hole(HoleProps { x_offset: 10, y_offset: 20, representation: 30, }).into_view();
        let rendered = hole.render_to_string().to_string();
        runtime.dispose();
        assert!(rendered.contains(r#"<g transform="translate(10,20)"#), "actual {}", rendered);
        assert!(rendered.contains(r#"<circle cx="2" cy="2" r="2" fill="red"#), "actual {}", rendered);
    }   
}