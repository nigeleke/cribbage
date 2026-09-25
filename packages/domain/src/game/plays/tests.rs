use std::collections::HashSet;

use macros::*;

use crate::{Card, Player, Value, constants::PLAY_TARGET};

use super::*;

#[test]
fn played_cards_is_empty_initially() {
    let play = PlayState::new(
        Player::Player0,
        [hand!("AH2H3H4H"), hand!("AD2D3D4D")].into(),
    );
    assert!(play.played_cards().is_empty());
}

#[test]
fn remaining_cards_contains_legal_cards_from_both_players() {
    let play = PlayState::new(Player::Player0, [hand!("2H5C"), hand!("3D7S")].into());
    assert_eq!(
        play.all_legal_plays(),
        vec![card!("2H"), card!("5C"), card!("3D"), card!("7S")]
    );
}

#[test]
fn remaining_cards_excludes_cards_that_exceed_31() {
    let mut play = PlayState::new(
        Player::Player1,
        [hand!("TH8H2H3C"), hand!("TCJCAH4D")].into(),
    );

    play.play(card!("TC"));
    play.play(card!("8H"));
    play.play(card!("JC"));
    play.play(card!("TH"));

    assert!(play.all_legal_plays().is_empty());
}

#[test]
fn played_cards_preserves_play_order() {
    let mut play = PlayState::new(
        Player::Player1,
        [hand!("TH8H2H3C"), hand!("TCJCAH4D")].into(),
    );

    play.play(card!("TC"));
    play.play(card!("8H"));
    play.play(card!("JC"));
    play.play(card!("TH"));

    assert_eq!(
        play.played_cards(),
        vec![card!("TC"), card!("8H"), card!("JC"), card!("TH"),]
    );
}

#[test]
fn remaining_cards_returns_legal_cards_for_both_players() {
    let play = PlayState::new(Player::Player0, [hand!("2H5C"), hand!("3D7S")].into());

    assert_eq!(
        play.all_legal_plays(),
        vec![card!("2H"), card!("5C"), card!("3D"), card!("7S"),]
    );
}

#[test]
fn play_reaches_target() {
    let mut state = PlayState::new(
        Player::Player1,
        [hand!("KDAHQSKS"), hand!("THJDQHKH")].into(),
    );

    state.play(card!("TH"));
    state.play(card!("KD"));
    state.play(card!("JD"));
    state.play(card!("AH"));

    assert_eq!(state.running_total(), Value::from(PLAY_TARGET));
    assert_eq!(state.next_to_play(), Player::Player1);
}

#[test]
fn play_player_continues_play() {
    let mut state = PlayState::new(Player::Player0, [hand!("5H"), hand!("6H")].into());

    state.play(card!("5H"));

    assert_eq!(state.running_total(), 5);
    assert_eq!(state.next_to_play(), Player::Player1);
}

#[test]
fn go_when_opponent_has_cards_calls_go_and_changes_player() {
    let mut play = PlayState::new(Player::Player0, [hand!("5H"), hand!("6C")].into());

    play.go();

    assert_eq!(*play.go_status(), GoStatus::Called);
    assert_eq!(play.next_to_play(), Player::Player1);
}

#[test]
fn go_after_go_ends_play() {
    let mut play = PlayState::new(Player::Player0, [hand!("5H"), hand!("6C")].into());

    play.go();
    play.go();

    assert_eq!(*play.go_status(), GoStatus::NotCalled);
    assert!(play.played_cards().is_empty());
}

#[test]
fn go_when_opponent_has_no_cards_ends_play() {
    let mut play = PlayState::new(Player::Player0, [hand!("5H"), hand!("")].into());

    play.go();

    assert_eq!(*play.go_status(), GoStatus::NotCalled);
    assert!(play.played_cards().is_empty());
}

#[test]
fn first_go_does_not_end_play_when_opponent_has_cards() {
    let mut play = PlayState::new(Player::Player0, [hand!("5H"), hand!("6C")].into());

    play.go();

    assert_eq!(*play.go_status(), GoStatus::Called);
    assert_eq!(play.next_to_play(), Player::Player1);
}
