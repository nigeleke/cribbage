use macros::*;

use super::*;

#[test]
fn new_scoreboard_has_zero_points() {
    let scoreboard = Scoreboard::default();

    assert_eq!(scoreboard.points(Player::Player0), Points::default());
    assert_eq!(scoreboard.points(Player::Player1), Points::default());
}

#[test]
fn record_score_increases_player_score() {
    let mut scoreboard = Scoreboard::default();

    let event = event!(try_pone_hand, Player0, &hand!("TH5H3C8D"), "AH");

    scoreboard.record_score(event);

    assert_eq!(scoreboard.points(Player::Player0), Points::from(2));
    assert_eq!(scoreboard.points(Player::Player1), Points::default());
}

#[test]
fn scores_are_accumulated() {
    let mut scoreboard = Scoreboard::default();

    let event0 = event!(try_dealer_hand, Player0, &hand!("TH5C3C8D"), "AH");
    let event1 = event!(try_crib, Player0, &crib!("JH5HQHKH"), "AH");

    scoreboard.record_score(event0);
    scoreboard.record_score(event1);

    assert_eq!(scoreboard.points(Player::Player0), Points::from(17));
}

#[test]
fn scores_are_tracked_independently_for_each_player() {
    let mut scoreboard = Scoreboard::default();

    let event0 = event!(try_pone_hand, Player0, &hand!("TH5H3C8D"), "AH");
    let event1 = event!(try_dealer_hand, Player1, &hand!("JH2C3C8D"), "6H");

    scoreboard.record_score(event0);
    scoreboard.record_score(event1);

    assert_eq!(scoreboard.points(Player::Player0), Points::from(2));
    assert_eq!(scoreboard.points(Player::Player1), Points::from(3));
}

#[test]
fn pegs_reflect_accumulated_score() {
    let mut scoreboard = Scoreboard::default();

    let event0 = event!(try_pone_hand, Player0, hand!("AH2H3H4H"), "5H");
    let event1 = event!(try_pone_hand, Player0, hand!("3C8H9DAS"), "5H");

    scoreboard.record_score(event0);
    scoreboard.record_score(event1);

    let pegs = scoreboard.pegs(Player::Player0);

    assert_eq!(pegs.front_peg(), Points::from(14));
    assert_eq!(pegs.back_peg(), Points::from(12));
}

#[test]
fn pegs_are_independent_for_each_player() {
    let mut scoreboard = Scoreboard::default();

    let event0 = event!(try_pone_hand, Player0, &hand!("TH5H3C8D"), "AH");
    let event1 = event!(try_pone_hand, Player0, &hand!("TH5H3C8D"), "AH");
    let event2 = event!(try_dealer_hand, Player1, &hand!("JH2C3C8D"), "6H");

    scoreboard.record_score(event0);
    scoreboard.record_score(event1);
    scoreboard.record_score(event2);

    let pegs0 = scoreboard.pegs(Player::Player0);
    let pegs1 = scoreboard.pegs(Player::Player1);

    assert_eq!(pegs0.back_peg(), Points::from(2));
    assert_eq!(pegs0.front_peg(), Points::from(4));

    assert_eq!(pegs1.back_peg(), Points::default());
    assert_eq!(pegs1.front_peg(), Points::from(3));
}

#[test]
fn winner_is_none_when_neither_player_has_reached_winning_score() {
    let mut scoreboard = Scoreboard::default();

    let event = event!(try_pone_hand, Player0, hand!("5H5C5DJS"), "5S");
    scoreboard.record_score(event);

    assert_eq!(scoreboard.winner(), None);
}

#[test]
fn winner_is_player_who_reaches_winning_score() {
    let mut scoreboard = Scoreboard::default();

    (0..5).for_each(|_| {
        let event = event!(try_pone_hand, Player0, hand!("5H5C5DJS"), "5S");
        scoreboard.record_score(event);
    });

    assert_eq!(scoreboard.winner(), Some(Player::Player0));
}

#[test]
fn winner_can_be_found_for_either_player() {
    let mut scoreboard = Scoreboard::default();

    (0..5).for_each(|_| {
        let event = event!(try_pone_hand, Player1, hand!("5H5C5DJS"), "5S");
        scoreboard.record_score(event);
    });

    assert_eq!(scoreboard.winner(), Some(Player::Player1));
}
