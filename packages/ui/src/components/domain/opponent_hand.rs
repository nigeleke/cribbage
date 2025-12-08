use api::dto::UserGameDTO;
use dioxus::prelude::*;

use crate::components::Cards;

/// The `Hand` component shows a set of cards in the order provided.
#[component]
pub fn OpponentHand() -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();
    let cards = use_memo(move || game().opponent_state.hand);

    rsx! {
        Cards { cards }
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
