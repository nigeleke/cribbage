use crate::components::Card;
use api::dto::{CardDTO, CardIdDTO, UserGameDTO};
use dioxus::prelude::*;

/// The `UserHand` component shows a set of cards (in the order provided).
/// If the 'on_click' event handle is provided then the cards will be selectable.
/// The children are any elements such as controls or other messages related to
/// the user's hand.
#[component]
pub fn UserHand(children: Element) -> Element {
    let game = use_context::<ReadSignal<UserGameDTO>>();
    let cards = use_memo(move || game().user_state.hand);

    let mut card_dtos = use_signal(Vec::<CardIdDTO>::default);
    let mut selections = use_signal(Vec::<bool>::default);
    let mut selection = use_signal(Option::<(usize, bool)>::default);
    let mut selected_cards = use_signal(Vec::<CardIdDTO>::default);

    use_effect(move || {
        card_dtos.set(
            cards()
                .iter()
                .filter_map(|c| match c {
                    CardDTO::FaceUp { cid } => Some(cid.clone()),
                    CardDTO::FaceDown => None, // user cards will always be face up
                })
                .collect::<Vec<_>>(),
        );
    });

    use_effect(move || selections.set(vec![false; card_dtos().len()]));

    use_effect(move || {
        if let Some((i, selected)) = *selection.read() {
            selections.write()[i] = selected;
        }
    });

    use_effect(move || {
        let dtos = card_dtos()
            .iter()
            .zip(selections())
            .filter_map(|(dto, selected)| selected.then_some(dto))
            .cloned()
            .collect::<Vec<_>>();
        selected_cards.set(dtos);
    });

    let on_card_selection = move |i: usize| {
        move |_| {
            let is_selected = selections.read()[i];
            selection.set(Some((i, !is_selected)));
        }
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/user_hand.css")},
        div {
            class: "user-hand",
            div {
                class: "user-hand__cards",
                for (i, card) in cards().into_iter().enumerate() {
                    Card {
                        card,
                        selected: *selections.read().get(i).unwrap_or(&false),
                        on_click: on_card_selection(i)
                    }
                }
            }
            div {
                class: "user-hand__controls",
                SelectedCardsProvider { selected_cards, children }
            }
        }
    }
}

#[component]
fn SelectedCardsProvider(selected_cards: ReadSignal<Vec<CardIdDTO>>, children: Element) -> Element {
    provide_context(selected_cards);
    rsx! { {children} }
}
