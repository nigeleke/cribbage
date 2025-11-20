use crate::components::{CardView, WaitingForOpponent};
use crate::route::Route;
use api::{CardDTO, GameIdDTO, PlayActionDTO, PlayerDTO, PlaysDTO, UserIdDTO};
use dioxus::prelude::*;

/// The `PlayingHand` component shows a set of cards (in the order provided).
#[component]
pub fn PlayingHand(
    cards: ReadSignal<Vec<CardDTO>>,
    plays: ReadSignal<Option<PlaysDTO>>,
) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = use_context::<GameIdDTO>();

    let card_cids = use_memo(move || {
        cards()
            .iter()
            .filter_map(|c| match c {
                CardDTO::FaceUp { cid } => Some(cid.clone()),
                CardDTO::FaceDown => unreachable!(),
            })
            .collect::<Vec<_>>()
    });

    let mut selections = use_signal(|| vec![false; cards().len()]);
    let mut selection = use_signal(|| None);
    let mut selected_count = use_signal(|| 0);

    use_effect(move || selections.set(vec![false; cards().len()]));

    use_effect(move || {
        if let Some((i, selected)) = *selection.read() {
            selections.write()[i] = selected;
        }
    });

    use_effect(move || {
        selected_count.set(
            selections
                .read()
                .iter()
                .map(|b| if *b { 1 } else { 0 })
                .sum(),
        )
    });

    let on_card_selection = move |i: usize| {
        move |_| {
            let users_turn = plays().as_ref().map_or(false, |plays| {
                plays.next_action == PlayActionDTO::Play(PlayerDTO::User)
            });

            let legal_play = plays().as_ref().map_or(false, |plays| {
                plays.legal_plays.contains(&card_cids.read()[i])
            });

            if users_turn && legal_play {
                let selected = selections.read()[i];
                selection.set(Some((i, !selected)))
            }
        }
    };

    let navigator = use_navigator();

    let on_play = move |_| {
        spawn(async move {
            let card = cards()
                .into_iter()
                .zip(selections())
                .filter_map(|(card, keep)| keep.then_some(card))
                .filter_map(|card| match card {
                    CardDTO::FaceUp { cid } => Some(cid),
                    CardDTO::FaceDown => None, // unreachable
                })
                .next()
                .expect("card must have been selected");

            match api::action::play_card(*user_id.read(), game_id, card).await {
                Ok(_) => {}
                Err(error) => {
                    warn!("GamePage:play:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            }
        });
    };

    let on_pass = move |_| {
        spawn(async move {
            match api::action::pass(*user_id.read(), game_id).await {
                Ok(_) => {}
                Err(error) => {
                    warn!("GamePage:pass:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            }
        });
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/playing_hand.css")},
        div {
            class: "playing-hand",
            div {
                for (i, card) in cards().into_iter().enumerate() {
                    CardView {
                        card,
                        selected: selections.read()[i],
                        on_click: on_card_selection(i)
                    }
                }
            }
            match plays().map(|plays| plays.next_action) {
                Some(PlayActionDTO::Play(player)) => rsx! {
                    if player == PlayerDTO::User {
                        button {
                            onclick: on_play,
                            disabled: selected_count() != 1,
                            "Play"
                        }
                    } else {
                        WaitingForOpponent {  }
                    }
                },
                Some(PlayActionDTO::Pass(player)) => rsx! {
                   if player == PlayerDTO::User {
                       button {
                           onclick: on_pass,
                           "Pass"
                       }
                   } else {
                       WaitingForOpponent { }
                   }
                },
                Some(PlayActionDTO::ScorePone) => rsx! {
                    button { "Score pone" }
                },
                _ => rsx! { p { } }
            }
        }
    }
}
