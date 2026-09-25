use macros::*;

use super::*;

#[test]
fn starter_his_heels() {
    let event = Event::try_starter(Player::Player0, card!("JH")).expect("valid event");
    assert_eq!(event.player(), Player::Player0);
    assert_eq!(event.points(), Points::from(2));
}

#[test]
fn starter_no_his_heels() {
    assert!(Event::try_starter(Player::Player0, card!("AH")).is_none());
}

#[test]
fn pone_hand() {
    let event = event!(try_pone_hand, Player0, hand!("TH5C3C8D"), "AH");
    assert_eq!(event.player(), Player::Player0);
    assert_eq!(event.points(), Points::from(2));
}

#[test]
fn dealer_hand() {
    let event = event!(try_dealer_hand, Player1, hand!("TH5C3C8D"), "AH");
    assert_eq!(event.player(), Player::Player1);
    assert_eq!(event.points(), Points::from(2));
}

#[test]
fn crib() {
    let event = event!(try_crib, Player0, crib!("TH5C3C8D"), "AH");
    assert_eq!(event.player(), Player::Player0);
    assert_eq!(event.points(), Points::from(2));
}

#[test]
fn hand_with_no_score() {
    let _ = Event::try_pone_hand(Player::Player0, &hand!("AH3C7D9S"), card!("KC")).is_none();
}

#[test]
fn cards_fifteens() {
    let cards = hand!("TH5C3C8D").to_vec();

    let calls = Event::cards_fifteens(&cards).collect::<Vec<_>>();

    assert_eq!(calls.len(), 1);
    assert_eq!(
        calls.iter().map(Call::points).sum::<Points>(),
        Points::from(2)
    );
}

#[test]
fn cards_pairs() {
    let cards = hand!("5H5C5D5S");

    let calls = Event::cards_pairs(&cards).collect::<Vec<_>>();

    assert_eq!(calls.len(), 6);
    assert_eq!(
        calls.iter().map(Call::points).sum::<Points>(),
        Points::from(12)
    );
}

#[test]
fn cards_runs_prefers_longest_run() {
    let cards = hand!("2H3C4D5S");

    let calls = Event::cards_runs(&cards).collect::<Vec<_>>();

    assert_eq!(calls.len(), 1);
    assert_eq!(
        calls.iter().map(Call::points).sum::<Points>(),
        Points::from(4)
    );
}

#[test]
fn cards_runs_returns_all_longest_runs() {
    let cards = hand!("2H3C4D4S");

    let calls = Event::cards_runs(&cards).collect::<Vec<_>>();

    assert_eq!(calls.len(), 2);
    assert_eq!(
        calls.iter().map(Call::points).sum::<Points>(),
        Points::from(6)
    );
}

#[test]
fn hand_flush_with_cut() {
    let cards = hand!("2H4H7H9H");

    let calls = Event::cards_flush(&cards, card!("KH"), FlushRule::Hand).collect::<Vec<_>>();

    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].points(), Points::from(5));
}

#[test]
fn hand_flush_without_cut() {
    let cards = hand!("2H4H7H9H");

    let calls = Event::cards_flush(&cards, card!("KC"), FlushRule::Hand).collect::<Vec<_>>();

    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].points(), Points::from(4));
}

#[test]
fn hand_no_flush() {
    let cards = hand!("2H4H7H9C");

    assert!(
        Event::cards_flush(&cards, card!("KH"), FlushRule::Hand)
            .next()
            .is_none()
    );
}

#[test]
fn crib_requires_five_card_flush() {
    let cards = crib!("2H4H7H9H");

    assert!(
        Event::cards_flush(&cards, card!("KC"), FlushRule::Crib)
            .next()
            .is_none()
    );
}

#[test]
fn crib_five_card_flush() {
    let cards = crib!("2H4H7H9H");

    let calls = Event::cards_flush(&cards, card!("KH"), FlushRule::Crib).collect::<Vec<_>>();

    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].points(), Points::from(5));
}

#[test]
fn cards_nobs() {
    let cards = hand!("JH2C4D9S");

    let calls = Event::cards_nobs(&cards, card!("AH")).collect::<Vec<_>>();

    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].points(), Points::from(1));
}

#[test]
fn cards_no_nobs() {
    let cards = hand!("JH2C4D9S");
    assert!(Event::cards_nobs(&cards, card!("AC")).next().is_none());
}
