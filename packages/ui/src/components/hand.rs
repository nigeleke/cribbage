use super::CardView;
use api::CardDTO;
use dioxus::prelude::*;

/// The `Hand` component shows a set of cards (in the order provided).
/// If `on_selected` is provided then it will be triggered when any
/// of the card's selected state changes.
#[component]
pub fn Hand(
    cards: Vec<CardDTO>,
    // #[prop(optional)] on_selected: Option<WriteSignal<Vec<bool>>>,
) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/hand.css")},
        div {
            class: "hand",
            for card in cards { CardView { card }  }
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
