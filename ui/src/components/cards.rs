use crate::view::CardSlot;

use super::card::Card;

use leptos::*;
use thaw::*;

/// The `Cards` component shows a set of cards (in the order provided).
/// The optional `stacked` setting will show the cards on top of each other if true.
/// The optional `opacity` setting will be passed to each card.
/// If `on_selected` is provided then it will be triggered when any
/// of the card's selected state changes.
#[component]
pub fn Cards(
    
    cards: Vec<CardSlot>,
    
    #[prop(optional)]
    stacked: bool,

    #[prop(optional)]
    opacity: String,
    
    #[prop(optional)]
    on_selected: Option<WriteSignal<Vec<bool>>>,

) -> impl IntoView {
    
    let selections = (0..cards.len()).map(|_| create_rw_signal(false)).collect::<Vec<_>>();
    let wo_selections = selections.iter().map(|s| s.write_only()).collect::<Vec<_>>();
    
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

    let children = {
        if cards.is_empty() {
            view! { <Card card=CardSlot::Empty /> }.into()
        } else if on_selected.is_none() {
            Fragment::from_iter(cards.iter()
                .take(n)
                .map(|card| view!{ <Card card={*card} opacity=opacity.clone() /> }))
        } else {
            Fragment::from_iter(cards.iter()
                .take(n)
                .enumerate()
                .map(|(i, card)| view!{ <Card card={*card} on_selected={wo_selections[i]} opacity=opacity.clone() /> }))
        }
    };

    let children = Box::new(|| children);

    view! { <Space children /> }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::Hand;
    use crate::test::LeptosRuntime;

    #[test]
    #[ignore]
    fn cards_rendered_when_face_up() {
        LeptosRuntime::new(
            || {
                let cards = Hand::from("AS2S3S")
                    .as_ref()
                    .into_iter()
                    .map(|c| CardSlot::FaceUp(*c))
                    .collect::<Vec<_>>();
                Cards(CardsProps { cards, stacked: false, opacity: "".into(), on_selected: None })
            },
            |_: &View| { println!("when");},
            |rendered: String| {
                println!("Cards::test: {}", rendered);
                assert!(rendered.contains(r#"<card-t rank="Ace" suit="Spades" opacity="""#));
                assert!(rendered.contains(r#"<card-t rank="Two" suit="Spades" opacity="""#));
                assert!(rendered.contains(r#"<card-t rank="Three" suit="Spades" opacity="""#));
            }
        ).run()
    }

    #[test]
    #[ignore]
    fn cards_rendered_when_face_down() {

    }

    #[test]
    #[ignore]
    fn cards_uses_vertical_space_when_cards_is_empty() {
    }
}