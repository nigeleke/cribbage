use super::CardView;
use api::CardDTO;
use dioxus::prelude::*;

/// The `DiscardingHand` component shows a set of cards (in the order provided).
/// `on_selected` will be triggered when a card is selected.
#[component]
pub fn DiscardingHand(
    cards: Vec<CardDTO>,
    // #[prop(optional)] on_selected: Option<WriteSignal<Vec<bool>>>,
) -> Element {
    let mut selections = use_signal(|| vec![false; cards.len()]);
    let mut selection = use_signal(|| None);

    use_effect(move || {
        if let Some((i, selected)) = *selection.read() {
            selections.write()[i] = selected;
        }
    });

    let on_click = move |i: usize| {
        move |_| {
            let selected = selections.read()[i];
            selection.set(Some((i, !selected)))
        }
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/hand.css")},
        div {
            class: "hand",
            for (i, card) in cards.into_iter().enumerate() {
                CardView { card, selected: selections.read()[i], on_click: on_click(i) }
            }
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    // use crate::domain::Hand;
    // use crate::test::LeptosRuntime;

    #[test]
    #[ignore]
    fn cards_rendered_when_face_up() {
        // LeptosRuntime::new(
        //     || {
        //         let cards = Hand::from("AS2S3S")
        //             .as_ref()
        //             .into_iter()
        //             .map(|c| CardSlot::FaceUp(*c))
        //             .collect::<Vec<_>>();
        //         Cards(CardsProps {
        //             cards,
        //             stacked: false,
        //             opacity: "".into(),
        //             on_selected: None,
        //         })
        //     },
        //     |_: &View| {
        //         println!("when");
        //     },
        //     |rendered: String| {
        //         println!("Cards::test: {}", rendered);
        //         assert!(rendered.contains(r#"<card-t rank="Ace" suit="Spades" opacity="""#));
        //         assert!(rendered.contains(r#"<card-t rank="Two" suit="Spades" opacity="""#));
        //         assert!(rendered.contains(r#"<card-t rank="Three" suit="Spades" opacity="""#));
        //     },
        // )
        // .run()
    }

    #[test]
    #[ignore]
    fn cards_rendered_when_face_down() {}

    #[test]
    #[ignore]
    fn cards_uses_vertical_space_when_cards_is_empty() {}
}
