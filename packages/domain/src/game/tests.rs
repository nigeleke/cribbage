mod game_fixture;

// ------------------------------------
use super::*;
use game_fixture::GameFixture;

/// # [Cribbage Rules](https://www.officialgamerules.org/cribbage)
///
/// ## Number of Players
///
/// Two or three people can play. Or four people can play two against two as partners. But
/// Cribbage is basically best played by two people, and the rules that follow are for that
/// number.
mod players {
    use super::*;
    use crate::PLAYERS;
    use pretty_assertions::assert_eq;

    #[test]
    fn two_players_participate() {
        assert_eq!(PLAYERS, [Player::Player0, Player::Player1]);
    }
}

/// ## The Pack
///
/// The standard 52-card pack is used.
///
/// Rank of Cards: K (high), Q, J, 10, 9, 8, 7, 6, 5, 4, 3, 2, A.
mod deck {
    use super::*;
    use crate::constants::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn use_a_standard_pack_of_cards() {
        let deck = Deck::new();
        assert_eq!(deck.len(), STANDARD_DECK_SIZE);
    }
}

/// ## The Draw, Shuffle and Cut
///
/// From a shuffled pack face down, each player cuts a card, leaving at least four cards at
/// either end of the pack.
///
/// If both players cut cards of the same rank, each draws again. The player with the lower card
/// deals the first hand. Thereafter, the turn to deal alternates between the two players,
/// except that the loser of the game deals first if another game is played. The dealer has the
/// right to shuffle last, and he presents the cards to the non-dealer for the cut prior to the
/// deal. (In some games, there is no cut at this time.)
mod deal_cut {
    use super::*;
    use macros::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn a_player_must_cut_for_dealer_1() {
        let given = GameFixture::default().with_deck("AH2H").as_starting();
        let outcome = given.cut_for_deal(Player::Player0, card!("AH"));

        match outcome {
            Ok(CutForDealOutcome::Starting(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_deck("2H")
                    .with_cuts([Some("AH"), None])
                    .as_starting()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn a_player_must_cut_for_dealer_2() {
        let given = GameFixture::default().with_deck("AH2H").as_starting();
        let outcome = given.cut_for_deal(Player::Player1, card!("AH"));

        match outcome {
            Ok(CutForDealOutcome::Starting(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_deck("2H")
                    .with_cuts([None, Some("AH")])
                    .as_starting()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn dealer_decided_with_lowest_cut_1() {
        let given = GameFixture::default()
            .with_deck("2H")
            .with_cuts([Some("AH"), None])
            .as_starting();
        let outcome = given.cut_for_deal(Player::Player1, card!("2H"));

        match outcome {
            Ok(CutForDealOutcome::Dealing(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_dealer(Player::Player0)
                    .as_dealing()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn dealer_decided_with_lowest_cut_2() {
        let given = GameFixture::default()
            .with_deck("2H")
            .with_cuts([None, Some("AH")])
            .as_starting();
        let outcome = given.cut_for_deal(Player::Player0, card!("2H"));

        match outcome {
            Ok(CutForDealOutcome::Dealing(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_dealer(Player::Player1)
                    .as_dealing()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn dealer_undecided_with_tied_cut() {
        let given = GameFixture::default()
            .with_deck("AD")
            .with_cuts([Some("AH"), None])
            .as_starting();
        let outcome = given.cut_for_deal(Player::Player1, card!("AD"));

        match outcome {
            Ok(CutForDealOutcome::Starting(actual)) => {
                assert_eq!(actual, GameFixture::default().with_deck("").as_starting())
            }
            other => panic!("unexpected state: {other:?}"),
        }
    }
}

/// ## The Deal
///
/// The dealer distributes six cards face down to his opponent and himself, beginning with the
/// opponent.
mod deal {
    use super::*;
    use macros::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn dealer_deals_six_cards_each() {
        let given = GameFixture::default().as_dealing();
        let outcome = given.deal(deck!("AH2H3H4H5H6HAD2D3D4D5D6DJH"));

        match outcome {
            Ok(DealOutcome::Discarding(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["AH2H3H4H5H6H", "AD2D3D4D5D6D"])
                    .with_deck("JH")
                    .as_discarding()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }
}

/// ## Object of the Game
///
/// The goal is to be the first player to score 121 points. (Some games are to 61 points.)
/// Players earn points during play and for making various card combinations.
mod object_of_the_game {}

/// ## The Crib
///
/// Each player looks at his six cards and "lays away" (discards) two of them face down to
/// reduce the hand to four. The four cards laid away together constitute "the crib". The crib
/// belongs to the dealer, but these cards are not exposed or used until after the hands have
/// been played.
#[allow(clippy::expect_used)]
mod the_crib {
    use super::*;
    use macros::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn player_can_discard_own_cards_to_the_crib() {
        let given = GameFixture::default()
            .with_hands(["AH2H3H4H5H6H", "AD2D3D4D5D6D"])
            .as_discarding();
        let outcome = given.discard(Player::Player0, [card!("AH"), card!("2H")].into());

        match outcome {
            Ok(DiscardOutcome::Discarding(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["3H4H5H6H", "AD2D3D4D5D6D"])
                    .with_discards([Some("AH2H"), None])
                    .as_discarding()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn player_cannot_discard_unowned_cards_to_the_crib() {
        let given = GameFixture::default()
            .with_hands(["AH2H3H4H5H6H", "AD2D3D4D5D6D"])
            .as_discarding();
        let outcome = given.discard(Player::Player0, [card!("AH"), card!("2D")].into());

        assert!(matches!(outcome, Err(GameError::CardsNotInHand)));
    }

    #[test]
    fn player_cannot_discard_if_already_discarded() {
        let given = GameFixture::default()
            .with_hands(["3H4H5H6H", "AD2D3D4D5D6D"])
            .with_discards([Some("AH2H"), None])
            .as_discarding();
        let outcome = given.discard(Player::Player0, [card!("AH"), card!("3H")].into());

        assert!(matches!(outcome, Err(GameError::PlayerAlreadyDiscarded)));
    }
}

/// ## Before the Play
///
/// After the crib is laid away, the non-dealer cuts the pack. The dealer turns up the top card
/// of the lower packet and places it face up on top of the pack. This card is the "starter." If
/// the starter is a jack, it is called "His Heels," and the dealer pegs (scores) 2 points at
/// once. The starter is not used in the play phase of Cribbage , but is used later for making
/// various card combinations that score points.
mod before_the_play {
    use super::*;
    use crate::Call;
    use macros::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn start_the_play_after_discards() {
        let given = GameFixture::default()
            .with_hands(["3H4H5H6H", "AD2D3D4D5D6D"])
            .with_discards([Some("AH2H"), None])
            .as_discarding();
        let outcome = given.discard(Player::Player1, [card!("AD"), card!("2D")].into());

        match outcome {
            Ok(DiscardOutcome::Cutting(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["3H4H5H6H", "3D4D5D6D"])
                    .with_crib("AH2HAD2D")
                    .as_cutting()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn not_score_his_heels_when_jack_is_not_cut() {
        let given = GameFixture::default().with_deck("KH").as_cutting();
        let outcome = given.cut_starter();

        match outcome {
            Ok(CutStarterOutcome::Playing(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_starter("KH")
                    .with_next_to_play(Player::Player1)
                    .as_playing()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn score_his_heels_when_jack_is_cut() {
        let given = GameFixture::default().with_deck("JH").as_cutting();
        let outcome = given.cut_starter();

        match outcome {
            Ok(CutStarterOutcome::Playing(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_starter("JH")
                    .with_next_to_play(Player::Player1)
                    .with_calls(Player::Player0, &[Call::hisheels(card!("JH"))])
                    .as_playing()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn score_his_heels_and_finish_game_when_jack_is_cut() {
        let given = GameFixture::default()
            .with_deck("JH")
            .at_120(Player::Player0)
            .as_cutting();
        let outcome = given.cut_starter();

        match outcome {
            Ok(CutStarterOutcome::Finished(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_starter("JH")
                    .at_120(Player::Player0)
                    .with_calls(Player::Player0, &[Call::hisheels(card!("JH"))])
                    .as_finished()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }
}

/// ## The Play
///
/// After the starter is turned, the non-dealer lays one of his cards face up on the table. The
/// dealer similarly exposes a card, then non-dealer again, and so on - the hands are exposed
/// card by card, alternately except for a "Go", as noted below. Each player keeps his
/// cards separate from those of his opponent.
///
/// As each person plays, he announces a running total of pips reached by the addition of the
/// last card to all those previously2 played. (Example: The non-dealer begins with a four,
/// saying "Four." The dealer plays a nine, saying "Thirteen".) The kings, queens and jacks
/// count 10 each; every other card counts its pip value (the ace counts one).
mod the_play {
    use super::*;
    use crate::{Call, Points};
    use macros::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn accept_valid_play() {
        let given = GameFixture::default().with_hands(["QH", "4C"]).as_playing();
        let outcome = given.play(Player::Player1, card!("4C"));

        match outcome {
            Ok(PlayOutcome::Playing(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["QH", ""])
                    .with_next_to_play(Player::Player0)
                    .with_current_plays(plays![(Player1, "4C")])
                    .as_playing()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn accept_valid_play_after_opponent_go_called() {
        let given = GameFixture::default()
            .with_hands(["9S", "4SAS"])
            .with_current_plays(plays![(Player1, "TC"), (Player0, "TD"), (Player0, "5C")])
            .with_go_status(GoStatus::Called)
            .as_playing();
        let outcome = given.play(Player::Player1, card!("4S"));

        match outcome {
            Ok(PlayOutcome::Playing(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["9S", "AS"])
                    .with_current_plays(plays![
                        (Player1, "TC"),
                        (Player0, "TD"),
                        (Player0, "5C"),
                        (Player1, "4S")
                    ])
                    .with_go_status(GoStatus::PlayContinued)
                    .as_playing()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn cannot_play_when_unheld_card() {
        let given = GameFixture::default().with_hands(["QH", "4C"]).as_playing();
        let outcome = given.play(Player::Player1, card!("QH"));

        match outcome {
            Err(GameError::CardsNotInHand) => (),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn cannot_play_when_not_their_turn() {
        let given = GameFixture::default().with_hands(["QH", "4C"]).as_playing();
        let outcome = given.play(Player::Player0, card!("QH"));

        match outcome {
            Err(GameError::PlayOutOfTurn) => (),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn cannot_play_when_play_exceeds_target() {
        let given = GameFixture::default()
            .with_hands(["QH", "4C"])
            .with_go_status(GoStatus::PlayContinued)
            .with_current_plays(plays![
                (Player1, "TC"),
                (Player0, "TD"),
                (Player0, "5C"),
                (Player1, "4S"),
            ])
            .as_playing();
        let outcome = given.play(Player::Player1, card!("4C"));

        match outcome {
            Err(GameError::InvalidPlay) => (),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn play_when_target_not_reached_mid_play() {
        let given = GameFixture::default()
            .with_hands(["5S", "5H"])
            .with_current_plays(plays![(Player0, "TH")])
            .as_playing();
        let outcome = given.play(Player::Player1, card!("5H"));

        match outcome {
            Ok(PlayOutcome::Playing(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["5S", ""])
                    .with_next_to_play(Player::Player0)
                    .with_current_plays(plays![(Player0, "TH"), (Player1, "5H")])
                    .with_calls(Player::Player1, &[Call::fifteen(&cards!("TH5H"))])
                    .as_playing()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn play_when_target_not_reached_end_play() {
        let given = GameFixture::default()
            .with_hands(["QS", "2H"])
            .with_current_plays(plays![(Player0, "JH"), (Player0, "2C")])
            .with_previous_plays(plays![
                (Player0, "7C"),
                (Player1, "6S"),
                (Player1, "2S"),
                (Player1, "KS")
            ])
            .as_playing();
        let outcome = given.play(Player::Player1, card!("2H"));

        match outcome {
            Ok(PlayOutcome::Playing(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["QS", ""])
                    .with_next_to_play(Player::Player0)
                    .with_current_plays(plays![(Player0, "JH"), (Player0, "2C"), (Player1, "2H")])
                    .with_previous_plays(plays![
                        (Player0, "7C"),
                        (Player1, "6S"),
                        (Player1, "2S"),
                        (Player1, "KS")
                    ])
                    .with_calls(Player::Player1, &[Call::pair(&cards!("2H2C"))])
                    .as_playing()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn play_when_target_not_reached_finished() {
        let given = GameFixture::default()
            .with_hands(["AH", "5H"])
            .with_current_plays(plays![(Player0, "JH")])
            .at_120(Player::Player1)
            .as_playing();
        let outcome = given.play(Player::Player1, card!("5H"));

        match outcome {
            Ok(PlayOutcome::Finished(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["AH", ""])
                    .with_next_to_play(Player::Player0)
                    .with_current_plays(plays![(Player0, "JH"), (Player1, "5H")])
                    .at_120(Player::Player1)
                    .with_calls(Player::Player1, &[Call::fifteen(&cards!("JH5H"))])
                    .as_finished_with_play_state()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn play_when_target_reached_mid_play() {
        let given = GameFixture::default()
            .with_hands(["6S", "AH"])
            .with_current_plays(plays![(Player0, "TH"), (Player0, "JH"), (Player0, "QH")])
            .with_previous_plays(plays![(Player0, "9H"), (Player1, "2S"), (Player1, "QS"),])
            .as_playing();
        let outcome = given.play(Player::Player1, card!("AH"));

        match outcome {
            Ok(PlayOutcome::Playing(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["6S", ""])
                    .with_next_to_play(Player::Player0)
                    .with_current_plays(plays![])
                    .with_previous_plays(plays![
                        (Player0, "9H"),
                        (Player1, "2S"),
                        (Player1, "QS"),
                        (Player0, "TH"),
                        (Player0, "JH"),
                        (Player0, "QH"),
                        (Player1, "AH")
                    ])
                    .with_calls(Player::Player1, &[Call::thirtyone(&cards!("THJHQHAH"))])
                    .as_playing()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn play_when_target_reached_end_play() {
        let given = GameFixture::default()
            .with_hands(["", "AH"])
            .with_current_plays(plays![(Player0, "TH"), (Player0, "JH"), (Player0, "QH")])
            .with_previous_plays(plays![
                (Player1, "2S"),
                (Player1, "QS"),
                (Player1, "6S"),
                (Player0, "QC"),
            ])
            .as_playing();
        let outcome = given.play(Player::Player1, card!("AH"));

        match outcome {
            Ok(PlayOutcome::Scoring(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["QCTHJHQH", "2SQS6SAH"])
                    .with_calls(Player::Player1, &[Call::thirtyone(&cards!("THJHQHAH"))])
                    .as_scoring_pone()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn play_when_target_reached_finished() {
        let given = GameFixture::default()
            .with_hands(["QC", "AH"])
            .with_current_plays(plays![(Player0, "TH"), (Player1, "JH"), (Player0, "QH")])
            .with_previous_plays(plays![(Player1, "9H"), (Player1, "5S"), (Player0, "6S"),])
            .at_120(Player::Player1)
            .as_playing();
        let outcome = given.play(Player::Player1, card!("AH"));

        match outcome {
            Ok(PlayOutcome::Finished(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["QC", ""])
                    .with_next_to_play(Player::Player0)
                    .with_current_plays(plays![])
                    .with_previous_plays(plays![
                        (Player1, "9H"),
                        (Player1, "5S"),
                        (Player0, "6S"),
                        (Player0, "TH"),
                        (Player1, "JH"),
                        (Player0, "QH"),
                        (Player1, "AH")
                    ])
                    .at_120(Player::Player1)
                    .with_calls(Player::Player1, &[Call::thirtyone(&cards!("THJHQHAH"))])
                    .as_finished_with_play_state()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn play_when_plays_finished_and_game_not_finished() {
        let given = GameFixture::default()
            .with_hands(["", "AH"])
            .with_current_plays(plays![(Player0, "8H"), (Player1, "JH"), (Player0, "QH")])
            .with_previous_plays(plays![
                (Player1, "9H"),
                (Player0, "4S"),
                (Player1, "5S"),
                (Player0, "6S")
            ])
            .as_playing();
        let outcome = given.play(Player::Player1, card!("AH"));

        match outcome {
            Ok(PlayOutcome::Scoring(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["4S6S8HQH", "9H5SJHAH"])
                    .with_calls(
                        Player::Player1,
                        &[Call::lastcard(&cards!("8HJHQHAH"), &cards!("9H4S5S6S"))],
                    )
                    .as_scoring_pone()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn play_when_plays_finished_and_game_finished() {
        let given = GameFixture::default()
            .with_hands(["", "AH"])
            .with_current_plays(plays![(Player0, "8H"), (Player1, "JH"), (Player0, "QH")])
            .with_previous_plays(plays![
                (Player1, "9H"),
                (Player0, "4S"),
                (Player1, "5S"),
                (Player0, "6S")
            ])
            .at_120(Player::Player1)
            .as_playing();
        let outcome = given.play(Player::Player1, card!("AH"));

        match outcome {
            Ok(PlayOutcome::Finished(actual)) => {
                assert_eq!(
                    actual,
                    GameFixture::default()
                        .with_hands(["", ""])
                        .with_current_plays(plays![
                            (Player0, "8H"),
                            (Player1, "JH"),
                            (Player0, "QH"),
                            (Player1, "AH")
                        ])
                        .with_previous_plays(plays![
                            (Player1, "9H"),
                            (Player0, "4S"),
                            (Player1, "5S"),
                            (Player0, "6S")
                        ])
                        .at_120(Player::Player1)
                        .with_calls(
                            Player::Player1,
                            &[Call::lastcard(&cards!("8HJHQHAH"), &cards!("9H4S5S6S"))],
                        )
                        .as_finished_with_play_state()
                )
            }
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn swap_player_after_pone_play() {
        let given = GameFixture::default()
            .with_hands(["7H8H8D9C", "4S5STHJH"])
            .as_playing();
        let outcome = given.play(Player::Player1, card!("4S"));

        match outcome {
            Ok(PlayOutcome::Playing(actual)) => {
                assert_eq!(actual.state.play_state.next_to_play(), Player::Player0)
            }
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn swap_player_after_dealer_play() {
        let given = GameFixture::default()
            .with_hands(["7H8H8D9C", "5STHJH"])
            .with_next_to_play(Player::Player0)
            .as_playing();
        let outcome = given.play(Player::Player0, card!("9C"));

        match outcome {
            Ok(PlayOutcome::Playing(actual)) => {
                assert_eq!(actual.state.play_state.next_to_play(), Player::Player1)
            }
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn reset_play_after_exact_target_reached() {
        let given = GameFixture::default()
            .with_hands(["7H8H8D", "5STH"])
            .with_next_to_play(Player::Player0)
            .with_current_plays(plays![(Player1, "JH"), (Player0, "9C"), (Player1, "4S")])
            .as_playing();
        let outcome = given.play(Player::Player0, card!("8H"));

        match outcome {
            Ok(PlayOutcome::Playing(actual)) => assert_eq!(
                actual,
                GameFixture::default()
                    .with_hands(["7H8D", "5STH"])
                    .with_previous_plays(plays![
                        (Player1, "JH"),
                        (Player0, "9C"),
                        (Player1, "4S"),
                        (Player0, "8H")
                    ])
                    .with_calls(Player::Player0, &[Call::thirtyone(&cards!("JH9C4S8H"))])
                    .as_playing()
            ),
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn score_play_points_for_fifteens() {
        let given = GameFixture::default()
            .with_hands(["KH", "8D"])
            .with_current_plays(plays![(Player0, "7D")])
            .as_playing();
        let outcome = given.play(Player::Player1, card!("8D"));

        match outcome {
            Ok(PlayOutcome::Playing(actual)) => {
                assert_eq!(actual.scoreboard.points(Player::Player1), Points::from(2))
            }
            other => panic!("unexpected state: {other:?}"),
        }
    }

    #[test]
    fn score_play_points_for_pair() {
        let given = GameFixture::default()
            .with_hands(["KH", "8D"])
            .with_current_plays(plays![(Player0, "8H")])
            .as_playing();
        let outcome = given.play(Player::Player1, card!("8D"));

        match outcome {
            Ok(PlayOutcome::Playing(actual)) => {
                assert_eq!(actual.scoreboard.points(Player::Player1), Points::from(2))
            }
            other => panic!("unexpected state: {other:?}"),
        }
    }
    //
    //      #[test]
    //      fn score_play_points_for_triplet() {
    //          game_test! {
    //              given: &scenario!(
    //                  build_playing(1);
    //                  with_points(0, 0),
    //                  with_cut("AS"),
    //                  with_hands("KH", "8DAH"),
    //                  with_current_plays(&[(1, "8C"), (0, "8S")])
    //              ),
    //              when: GameCommand::PlayCard {
    //                  player: PLAYER1,
    //                  card: card!("8D"),
    //              },
    //              then_events: |events: &[GameEvent]| {
    //                  assert_eq!(events, &[GameEvent::CardPlayed {
    //                      player: PLAYER1,
    //                      card: card!("8D"),
    //                      pegging: Pegging::new(
    //                          PLAYER1,
    //                          ScoreSheet::default().add_event(
    //                              ScoreKind::Triplet,
    //                              &cards!("8D8S8C"),
    //                              Points::from(6),
    //                          ),
    //                      ),
    //                  }])
    //              }
    //          }
    //      }
    //
    //      #[test]
    //      fn score_play_points_for_quartet() {
    //          game_test! {
    //              given: &scenario!(
    //                  build_playing(1);
    //                  with_points(0, 0),
    //                  with_cut("AS"),
    //                  with_hands("KH", "7DAH"),
    //                  with_current_plays(&[(1, "7C"), (0, "7S"), (0, "7H")])
    //              ),
    //              when: GameCommand::PlayCard {
    //                  player: PLAYER1,
    //                  card: card!("7D"),
    //              },
    //              then_events: |events: &[GameEvent]| {
    //                  assert_eq!(events, &[GameEvent::CardPlayed {
    //                      player: PLAYER1,
    //                      card: card!("7D"),
    //                      pegging: Pegging::new(
    //                          PLAYER1,
    //                          ScoreSheet::default().add_event(
    //                              ScoreKind::Quadruplet,
    //                              &cards!("7D7H7S7C"),
    //                              Points::from(12),
    //                          ),
    //                      ),
    //                  }])
    //              }
    //          }
    //      }
    //
    //      #[test]
    //      fn score_play_points_for_run() {
    //          game_test! {
    //              given: &scenario!(
    //                  build_playing(1);
    //                  with_points(0, 0),
    //                  with_cut("AC"),
    //                  with_hands("KH", "AS"),
    //                  with_current_plays(&[(1, "2D"), (0, "3H")])
    //              ),
    //              when: GameCommand::PlayCard {
    //                  player: PLAYER1,
    //                  card: card!("AS"),
    //              },
    //              then_events: |events: &[GameEvent]| {
    //                  assert_eq!(events, &[GameEvent::CardPlayed {
    //                      player: PLAYER1,
    //                      card: card!("AS"),
    //                      pegging: Pegging::new(
    //                          PLAYER1,
    //                          ScoreSheet::default().add_event(
    //                              ScoreKind::Run,
    //                              &cards!("AS2D3H"),
    //                              Points::from(3),
    //                          ),
    //                      ),
    //                  }])
    //              }
    //          }
    //      }
    //
    //      #[test]
    //      fn score_play_points_for_run_edge_case_1() {
    //          game_test! {
    //              given: &scenario!(
    //                  build_playing(0);
    //                  with_points(0, 0),
    //                  with_cut("AS"),
    //                  with_hands("7H6H", "AH"),
    //                  with_current_plays(&[(1, "8S"), (0, "7H"), (1, "7S")])
    //              ),
    //              when: GameCommand::PlayCard {
    //                  player: PLAYER0,
    //                  card: card!("6H"),
    //              },
    //              then_events: |events: &[GameEvent]| {
    //                  assert_eq!(events, &[GameEvent::CardPlayed {
    //                      player: PLAYER0,
    //                      card: card!("6H"),
    //                      pegging: Pegging::new(PLAYER0, ScoreSheet::default()),
    //                  }])
    //              }
    //          }
    //      }
    //
    //      #[test]
    //      fn score_play_points_for_run_edge_case_2() {
    //          game_test! {
    //              given: &scenario!(
    //                  build_playing(0);
    //                  with_points(0, 0),
    //                  with_cut("AS"),
    //                  with_hands("5H7H", "AH"),
    //                  with_current_plays(&[(1, "9S"), (0, "6H"), (1, "8S")])
    //              ),
    //              when: GameCommand::PlayCard {
    //                  player: PLAYER0,
    //                  card: card!("7H"),
    //              },
    //              then_events: |events: &[GameEvent]| {
    //                  assert_eq!(events, &[GameEvent::CardPlayed {
    //                      player: PLAYER0,
    //                      card: card!("7H"),
    //                      pegging: Pegging::new(
    //                          PLAYER0,
    //                          ScoreSheet::default().add_event(
    //                              ScoreKind::Run,
    //                              &cards!("6H7H8S9S"),
    //                              Points::from(4),
    //                          ),
    //                      ),
    //                  }])
    //              }
    //          }
    //      }
    //
    //      #[test]
    //      fn regather_played_cards_after_winning_play() {
    //          game_test! {
    //              given: &scenario!(
    //                  build_playing(0);
    //                  with_points(115, 116),
    //                  with_cut("9D"),
    //                  with_hands("5C", "4S"),
    //                  with_previous_plays(&[(0, "JH"), (1, "KD"), (0, "TH")]),
    //                  with_current_plays(&[(1, "6D"), (0, "5S"), (1, "5D")])
    //              ),
    //              when: GameCommand::PlayCard {
    //                  player: PLAYER0,
    //                  card: card!("5C"),
    //              },
    //              then_events: |events: &[GameEvent]| {
    //                  assert_eq!(events, &[GameEvent::CardPlayed {
    //                      player: PLAYER0,
    //                      card: card!("5C"),
    //                      pegging: Pegging::new(
    //                          PLAYER0,
    //                          ScoreSheet::default().add_event(
    //                              ScoreKind::Triplet,
    //                              &cards!("5C5D5S"),
    //                              Points::from(6),
    //                          ),
    //                      ),
    //                  }])
    //              },
    //              then_phase: |phase: &Phase| {
    //                  assert_phase_then!(phase, Phase::Finished(finished) => {
    //                      assert_eq!(finished.hand(PLAYER0).clone().sorted(), hand!("JHTH5S5C"));
    //                      assert_eq!(finished.hand(PLAYER1).clone().sorted(), hand!("KD6D5D4S"));
    //                  });
    //              }
    //          }
    //      }
}
//
//  /// ## The Go
//  ///
//  /// During play, the running total of cards may never be carried beyond 31. If a player cannot
//  /// add another card without exceeding 31, he or she says "Go" and the opponent pegs 1. After
//  /// gaining the Go, the opponent must first lay down any additional cards he can without
//  /// exceeding 31. Besides the point for Go, he may then score any additional points that can be
//  /// made through pairs and runs (described later). If a player reaches exactly 31, he pegs two
//  /// instead of one for Go.
//  ///
//  /// The player who called Go leads for the next series of plays, with the count starting at
//  /// zero. The lead may not be combined with any cards previously played to form a scoring
//  /// combination; the Go has interrupted the sequence.
//  ///
//  /// The person who plays the last card pegs one for Go, plus one extra if the card brings the
//  /// count to exactly 31. The dealer is sure to peg at least one point in every hand, for he will
//  /// have a Go on the last card if not earlier.
//  #[allow(clippy::expect_used)]
//  mod the_go {
//      use super::*;
//
//      #[test]
//      fn accept_go_when_pone_has_no_valid_card() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(1);
//                  with_points(0, 0),
//                  with_cut("AS"),
//                  with_hands("AH", "KH"),
//                  with_current_plays(&[(0, "TH"), (0, "JH"), (0, "QH")])
//              ),
//              when: GameCommand::Go { player: PLAYER1 },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events, &[GameEvent::GoCalled {
//                      player: PLAYER1,
//                      pegging: Pegging::new(PLAYER0, ScoreSheet::default()),
//                  }])
//              }
//          }
//      }
//
//      #[test]
//      fn accept_go_when_dealer_has_no_valid_card() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(0);
//                  with_go(),
//                  with_points(0, 0),
//                  with_cut("AS"),
//                  with_hands("KH", "KS"),
//                  with_current_plays(&[(0, "TH"), (1, "QH"), (0, "JH")])
//              ),
//              when: GameCommand::Go { player: PLAYER0 },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events, &[GameEvent::GoCalled {
//                      player: PLAYER0,
//                      pegging: Pegging::new(
//                          PLAYER0,
//                          ScoreSheet::default().add_event(ScoreKind::LastCard, &cards!("JH"), Points::from(1)),
//                      ),
//                  }])
//              }
//          }
//      }
//
//      #[test]
//      fn cannot_call_go_when_valid_card_held() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(1);
//                  with_points(0, 0),
//                  with_cut("AC"),
//                  with_hands("AH", "AS"),
//                  with_current_plays(&[(0, "TH"), (0, "JH"), (0, "8H")])
//              ),
//              when: GameCommand::Go { player: PLAYER1 },
//              then_error: DomainError::InvalidGo
//          }
//      }
//
//      #[test]
//      fn cannot_call_go_when_not_turn() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(1);
//                  with_points(0, 0),
//                  with_cut("AC"),
//                  with_hands("AH", "AS"),
//                  with_current_plays(&[(0, "TH"), (0, "JH"), (0, "8H")])
//              ),
//              when: GameCommand::Go { player: PLAYER0 },
//              then_error: DomainError::NotPlayersTurn(PLAYER0)
//          }
//      }
//
//      #[test]
//      fn score_go_when_both_players_called_go_playing() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(0);
//                  with_go(),
//                  with_points(0, 0),
//                  with_cut("AS"),
//                  with_hands("KH", "KS"),
//                  with_current_plays(&[(0, "TH"), (1, "QH"), (0, "JH")])
//              ),
//              when: GameCommand::Go { player: PLAYER0 },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events, &[GameEvent::GoCalled {
//                      player: PLAYER0,
//                      pegging: Pegging::new(
//                          PLAYER0,
//                          ScoreSheet::default().add_event(ScoreKind::LastCard, &cards!("JH"), Points::from(1)),
//                      ),
//                  }])
//              }
//          }
//      }
//
//      #[test]
//      fn score_go_when_both_players_called_go_finished() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(0);
//                  with_go(),
//                  with_points(120, 0),
//                  with_cut("AS"),
//                  with_hands("KH", "KS"),
//                  with_current_plays(&[(0, "TH"), (1, "QH"), (0, "JH")])
//              ),
//              when: GameCommand::Go { player: PLAYER0 },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events, &[GameEvent::GoCalled {
//                      player: PLAYER0,
//                      pegging: Pegging::new(
//                          PLAYER0,
//                          ScoreSheet::default().add_event(ScoreKind::LastCard, &cards!("JH"), Points::from(1)),
//                      ),
//                  }])
//              },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Finished(finished) => {
//                      assert_eq!(finished.winner(), PLAYER0);
//                  })
//              }
//          }
//      }
//
//      #[test]
//      fn score_go_when_played_last_card_and_opponent_calls_go_1() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(0);
//                  with_points(0, 0),
//                  with_cut("QH"),
//                  with_hands("KH", ""),
//                  with_previous_plays(&[(1, "KC"), (0, "TC"), (1, "JS")]),
//                  with_current_plays(&[(0, "9C"), (1, "4C"), (0, "TS"), (1, "6D")])
//              ),
//              when: GameCommand::Go { player: PLAYER0 },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events, &[GameEvent::GoCalled {
//                      player: PLAYER0,
//                      pegging: Pegging::new(
//                          PLAYER1,
//                          ScoreSheet::default().add_event(ScoreKind::LastCard, &cards!("6D"), Points::from(1)),
//                      ),
//                  }])
//              },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Playing(playing) => {
//                      assert_eq!(playing.play_state().next_to_play(), PLAYER0);
//                      assert_eq!(playing.scoreboard().latest_pegging(), Some(&Pegging::new(PLAYER1, ScoreSheet::default().add_event(ScoreKind::LastCard, &cards!("6D"), Points::from(1)))));
//                  })
//              }
//          }
//      }
//
//      #[tracing_test::traced_test]
//      #[test]
//      fn score_go_when_played_last_card_and_opponent_calls_go_2() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(0);
//                  with_go(),
//                  with_points(0, 0),
//                  with_cut("TS"),
//                  with_hands("", "TH"),
//                  with_previous_plays(&[(1, "KS"), (0, "7S"), (1, "JD")]),
//                  with_current_plays(&[(0, "7D"), (1, "JS"), (0, "5H"), (0, "5S")])
//              ),
//              when: GameCommand::Go { player: PLAYER0 },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events, &[GameEvent::GoCalled {
//                      player: PLAYER0,
//                      pegging: Pegging::new(
//                          PLAYER0,
//                          ScoreSheet::default().add_event(ScoreKind::LastCard, &cards!("5S"), Points::from(1)),
//                      ),
//                  }])
//              },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Playing(playing) => {
//                      assert_eq!(playing.play_state().next_to_play(), PLAYER1);
//                      assert_eq!(playing.scoreboard().latest_pegging(), Some(&Pegging::new(PLAYER0, ScoreSheet::default().add_event(ScoreKind::LastCard, &cards!("5S"), Points::from(1)))));
//                  })
//              }
//          }
//      }
//
//      #[test]
//      fn swap_player_after_pone_called_go() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(1);
//                  with_points(0, 0),
//                  with_cut("AS"),
//                  with_hands("8H8D", "5SJH"),
//                  with_current_plays(&[(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
//              ),
//              when: GameCommand::Go { player: PLAYER1 },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Playing(playing) => {
//                      assert_eq!(playing.play_state().next_to_play(), PLAYER0)
//                  })
//              }
//          }
//      }
//
//      #[test]
//      fn swap_player_after_dealer_called_go() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(0);
//                  with_points(0, 0),
//                  with_cut("AS"),
//                  with_hands("7H8H8D", "4S5S"),
//                  with_current_plays(&[(1, "JH"), (0, "9C"), (1, "TH")])
//              ),
//              when: GameCommand::Go { player: PLAYER0 },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Playing(playing) => {
//                      assert_eq!(playing.play_state().next_to_play(), PLAYER1)
//                  })
//              }
//          }
//      }
//
//      #[test]
//      fn reset_play_after_pone_then_dealer_called_go() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(0);
//                  with_go(),
//                  with_points(0, 0),
//                  with_cut("AS"),
//                  with_hands("8H8D", "5SJH"),
//                  with_current_plays(&[(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
//              ),
//              when: GameCommand::Go { player: PLAYER0 },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Playing(playing) => {
//                      assert_eq!(playing.dealer(), &Dealer::from(PLAYER0));
//                      assert_eq!(playing.pone(), &Pone::from(PLAYER1));
//                      assert_eq!(playing.play_state().next_to_play(), PLAYER1);
//                      assert_eq!(
//                          playing.play_state().previous_plays(),
//                          &plays!(&[(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
//                      );
//                      assert!(playing.play_state().current_plays().is_empty());
//                  })
//              }
//          }
//      }
//
//      #[test]
//      fn reset_play_after_after_dealer_then_pone_called_go() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(1);
//                  with_go(),
//                  with_points(0, 0),
//                  with_cut("AS"),
//                  with_hands("7H8H8D", "4S5S"),
//                  with_current_plays(&[(1, "JH"), (0, "9C"), (1, "TH")])
//              ),
//              when: GameCommand::Go { player: PLAYER1 },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Playing(playing) => {
//                      assert_eq!(playing.dealer(), &Dealer::from(PLAYER0));
//                      assert_eq!(playing.pone(), &Pone::from(PLAYER1));
//                      assert_eq!(playing.play_state().next_to_play(), PLAYER0);
//                      assert_eq!(
//                          playing.play_state().previous_plays(),
//                          &plays!(&[(1, "JH"), (0, "9C"), (1, "TH")])
//                      );
//                      assert!(playing.play_state().current_plays().is_empty());
//                  })
//              }
//          }
//      }
//
//      #[test]
//      fn regather_played_cards_after_winning_go() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(1);
//                  with_go(),
//                  with_points(115, 120),
//                  with_cut("9D"),
//                  with_hands("5C", "4S"),
//                  with_previous_plays(&[(0, "JH"), (1, "KD"), (0, "TH")]),
//                  with_current_plays(&[(1, "TC"), (0, "TS"), (1, "TD")])
//              ),
//              when: GameCommand::Go {
//                  player: PLAYER1,
//              },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events, &[GameEvent::GoCalled {
//                      player: PLAYER1,
//                      pegging: Pegging::new(
//                          PLAYER1,
//                          ScoreSheet::default().add_event(
//                              ScoreKind::LastCard,
//                              &cards!("TD"),
//                              Points::from(1),
//                          ),
//                      ),
//                  }])
//              },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Finished(finished) => {
//                      assert_eq!(finished.hand(PLAYER0).clone().sorted(), hand!("JHTSTH5C"));
//                      assert_eq!(finished.hand(PLAYER1).clone().sorted(), hand!("KDTDTC4S"));
//                  });
//              }
//          }
//      }
//  }
//
//  /// ## Pegging
//  ///
//  /// The object in play is to score points by pegging. In addition to a Go, a player may score
//  /// for the following combinations:
//  ///
//  ///   - Fifteen: For adding a card that makes the total 15 Peg 2
//  ///   - Pair: For adding a card of the same rank as the card just played Peg 2 (Note that face
//  ///     cards pair only by actual rank: jack with jack, but not jack with queen.)
//  ///   - Triplet: For adding the third card of the same rank. Peg 6
//  ///   - Four: (also called "Double Pair" or "Double Pair Royal") For adding the fourth card of
//  ///     the same rank Peg 12
//  ///   - Run (Sequence): For adding a card that forms, with those just played:
//  ///     - For a sequence of three Peg 3
//  ///     - For a sequence of four. Peg 4
//  ///     - For a sequence of five. Peg 5
//  ///     - (Peg one point more for each extra card of a sequence. Note that runs are independent
//  ///       of suits, but go strictly by rank; to illustrate: 9, 10, J, or J, 9, 10 is a run but
//  ///       9, 10, Q is not)
//  ///
//  /// It is important to keep track of the order in which cards are played to determine whether
//  /// what looks like a sequence or a run has been interrupted by a "foreign card." Example:
//  /// Cards are played in this order: 8, 7, 7, 6. The dealer pegs 2 for 15, and the opponent
//  /// pegs 2 for pair, but the dealer cannot peg for run because of the extra seven (foreign
//  /// card) that has been played. Example: Cards are played in this order: 9, 6, 8, 7. The
//  /// dealer pegs 2 for fifteen when he plays the six and pegs 4 for run when he plays the seven
//  /// (the 6, 7, 8, 9 sequence). The cards were not played in sequential order, but they form a
//  /// true run with no foreign card.
//  #[allow(clippy::expect_used)]
//  mod pegging {
//      use super::*;
//
//      #[test]
//      fn should_score_fifteens() {
//          let Phase::Playing(playing) = Game::from(
//              scenario!(
//                  build_playing(1);
//                  with_points(0, 0),
//                  with_hands("AC", ""),
//                  with_current_plays(&[(0, "JD"), (0, "5H")]),
//                  with_cut("AH")
//              )
//              .as_slice(),
//          )
//          .phase
//          else {
//              panic!("unexpected state");
//          };
//
//          assert_eq!(
//              ScoreSheet::play_card(playing.play_state()).points(),
//              Points::from(2)
//          )
//      }
//
//      #[test]
//      fn should_score_pairs() {
//          let Phase::Playing(playing) = Game::from(
//              scenario!(
//                  build_playing(1);
//                  with_points(0, 0),
//                  with_hands("AC", ""),
//                  with_current_plays(&[(0, "JD"), (0, "AH"), (0, "AS")]),
//                  with_cut("KH")
//              )
//              .as_slice(),
//          )
//          .phase
//          else {
//              panic!("unexpected state");
//          };
//
//          assert_eq!(
//              ScoreSheet::play_card(playing.play_state()).points(),
//              Points::from(2)
//          )
//      }
//
//      #[test]
//      fn should_score_royal_pairs() {
//          let Phase::Playing(playing) = Game::from(
//              scenario!(
//                  build_playing(1);
//                  with_points(0, 0),
//                  with_hands("AC", ""),
//                  with_current_plays(&[(0, "AD"), (0, "AH"), (0, "AS")]),
//                  with_cut("KH")
//              )
//              .as_slice(),
//          )
//          .phase
//          else {
//              panic!("unexpected state");
//          };
//
//          assert_eq!(
//              ScoreSheet::play_card(playing.play_state()).points(),
//              Points::from(6)
//          )
//      }
//
//      #[test]
//      fn should_score_double_royal_pairs() {
//          let Phase::Playing(playing) = Game::from(
//              scenario!(
//                  build_playing(1);
//                  with_points(0, 0),
//                  with_hands("2H", ""),
//                  with_current_plays(&[(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")]),
//                  with_cut("KH")
//              )
//              .as_slice(),
//          )
//          .phase
//          else {
//              panic!("unexpected state")
//          };
//
//          assert_eq!(
//              ScoreSheet::play_card(playing.play_state()).points(),
//              Points::from(12)
//          )
//      }
//
//      #[test]
//      fn should_score_runs() {
//          let current_plays = &[
//              (0, "2C"),
//              (0, "3C"),
//              (0, "4C"),
//              (0, "5C"),
//              (0, "6C"),
//              (0, "7C"),
//          ];
//
//          for len in 1..=current_plays.len() {
//              let current_plays = *current_plays;
//              let current_plays = current_plays.into_iter().take(len);
//              let current_plays = Vec::from_iter(current_plays);
//              let Phase::Playing(playing) = Game::from(
//                  scenario!(
//                      build_playing(1);
//                      with_points(0, 0),
//                      with_hands("AS", "AD"),
//                      with_current_plays(&current_plays),
//                      with_cut("KH")
//                  )
//                  .as_slice(),
//              )
//              .phase
//              else {
//                  panic!("unexpected state")
//              };
//
//              assert_eq!(
//                  ScoreSheet::play_card(playing.play_state()).points(),
//                  Points::from(if len < 3 { 0 } else { len })
//              )
//          }
//      }
//
//      #[test]
//      fn should_score_runs_unordered() {
//          let Phase::Playing(playing) = Game::from(
//              scenario!(
//                  build_playing(1);
//                  with_points(0, 0),
//                  with_hands("KS", "KD"),
//                  with_current_plays(&[(0, "3S"), (0, "2C"), (0, "AS")]),
//                  with_cut("KH")
//              )
//              .as_slice(),
//          )
//          .phase
//          else {
//              panic!("unexpected state");
//          };
//
//          assert_eq!(
//              ScoreSheet::play_card(playing.play_state()).points(),
//              Points::from(3)
//          )
//      }
//
//      #[test]
//      fn should_score_rules_example_flush() {
//          let Phase::Playing(playing) = Game::from(
//              scenario!(
//                  build_playing(0);
//                  with_points(0, 0),
//                  with_hands("", "2H"),
//                  with_cut("3H"),
//                  with_current_plays(&[(1, "TH"), (0, "8H"), (1, "QH"), (0, "AH")])
//              )
//              .as_slice(),
//          )
//          .phase
//          else {
//              panic!("unexpected state");
//          };
//
//          assert_eq!(
//              ScoreSheet::play_card(playing.play_state()).points(),
//              Points::from(0)
//          );
//      }
//
//      #[test]
//      fn should_score_when_target_not_reached() {
//          let Phase::Playing(playing) = Game::from(
//              scenario!(
//                  build_playing(1);
//                  with_go(),
//                  with_points(0, 0),
//                  with_hands("", ""),
//                  with_current_plays(&[(0, "AC"), (0, "2D"), (0, "5H"), (0, "4S")]),
//                  with_cut("KH")
//              )
//              .as_slice(),
//          )
//          .phase
//          else {
//              panic!("unexpected state");
//          };
//
//          assert_eq!(
//              ScoreSheet::go(playing.play_state()).points(),
//              Points::from(1)
//          );
//      }
//
//      #[test]
//      fn should_score_when_target_reached() {
//          let Phase::Playing(playing) = Game::from(
//              scenario!(
//                  build_playing(1);
//                  with_points(0, 0),
//                  with_hands("", ""),
//                  with_current_plays(&[(0, "KC"), (0, "KD"), (0, "KH"), (0, "AS")]),
//                  with_cut("KS")
//              )
//              .as_slice(),
//          )
//          .phase
//          else {
//              panic!("unexpected state");
//          };
//
//          assert_eq!(
//              ScoreSheet::play_card(playing.play_state()).points(),
//              Points::from(2)
//          )
//      }
//  }
//
//  /// ## Counting the Hands
//  ///
//  /// When play ends, the three hands are counted in order: non-dealer's hand (first), dealer's
//  /// hand (second), and then the crib (third). This order is important because, toward the end of
//  /// a game, the non-dealer may "count out" and win before the dealer has a chance to count, even
//  /// though the dealer's total would have exceeded that of the opponent. The starter is
//  /// considered to be a part of each hand, so that all hands in counting comprise five cards. The
//  /// basic scoring formations are as follows:
//  ///
//  /// Combinations counts
//  ///   - Fifteen. Each combination of cards that totals 15 2
//  ///   - Pair. Each pair of cards of the same rank 2
//  ///   - Run. Each combination of three or more 1 cards in sequence (for each card in the
//  ///     sequence)
//  ///   - Flush.
//  ///     - Four cards of the same suit in hand 4 (excluding the crib, and the starter)
//  ///     - Four cards in hand or crib of the same 5 suit as the starter. (There is no count for
//  ///       four-flush in the crib that is not of same suit as the starter)
//  ///   - His Nobs. Jack of the same suit as starter in hand or crib 1
//  #[allow(clippy::expect_used)]
//  mod counting_the_hands {
//      use super::*;
//
//      #[test]
//      fn score_pone_hand_when_plays_finished() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(1);
//                  with_points(0, 0),
//                  with_hands("", "TH"),
//                  with_cut("4H"),
//                  with_previous_plays(&[
//                      (0, "7H"), (0, "8C"), (0, "AC"), (0, "2C"),
//                      (1, "QH"), (1, "KS"), (1, "5H"), (1, "TH"),
//                  ]),
//                  with_ack(0)
//              ),
//              when: GameCommand::ScorePone { player: PLAYER1 },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events.len(), 1);
//
//                  find_then!(events, GameEvent::PoneScored { player, pegging } => {
//                      assert_eq!(player, &PLAYER1);
//                      assert_eq!(pegging.recipient(), &PLAYER1);
//                      assert_eq!(pegging.score_sheet().points(), Points::from(6));
//                  });
//              }
//          }
//      }
//
//      #[test]
//      fn score_winning_pone_hand_when_plays_finished() {
//          game_test! {
//              given: &scenario!(
//                  build_playing(1);
//                  with_points(0, 115),
//                  with_hands("", "TH"),
//                  with_cut("4H"),
//                  with_previous_plays(&[
//                      (0, "7H"), (0, "8C"), (0, "AC"), (0, "2C"),
//                      (1, "QH"), (1, "KS"), (1, "5H"), (1, "TH"),
//                  ]),
//                  with_ack(0)
//              ),
//              when: GameCommand::ScorePone { player: PLAYER1 },
//              then_events: |events: &[GameEvent]| {
//                  find_then!(events, GameEvent::PoneScored { player, pegging } => {
//                      assert_eq!(player, &PLAYER1);
//                      assert_eq!(pegging.recipient(), &PLAYER1);
//                      assert_eq!(pegging.score_sheet().points(), Points::from(6));
//                  });
//              },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Finished(finished) => {
//                      assert_eq!(finished.winner(), PLAYER1);
//                  });
//              }
//          }
//      }
//
//      #[test]
//      fn score_dealer_hand_when_pone_score_acknowledged() {
//          game_test! {
//              given: &scenario!(
//                  build_scoring_pone;
//                  with_points(0, 0),
//                  with_cut("4H"),
//                  with_hands("7H8CAC2C", "JCKS5HTH"),
//                  with_crib("AHADASTD"),
//                  with_ack(0),
//              ),
//              when: GameCommand::ScoreDealer { player: PLAYER1 },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events.len(), 1);
//
//                  find_then!(events, GameEvent::DealerScored { player, pegging } => {
//                      assert_eq!(player, &PLAYER1);
//                      assert_eq!(pegging.recipient(), &PLAYER0);
//                      assert_eq!(pegging.score_sheet().points(), Points::from(4));
//                  });
//              }
//          }
//      }
//
//      #[test]
//      fn score_winning_dealer_hand_when_pone_score_acknowledged() {
//          game_test! {
//              given: &scenario!(
//                  build_scoring_pone;
//                  with_points(117, 0),
//                  with_cut("4H"),
//                  with_hands("7H8CAC2C", "JCKS5HTH"),
//                  with_crib("AHADASTD"),
//                  with_ack(0),
//              ),
//              when: GameCommand::ScoreDealer { player: PLAYER1 },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events.len(), 1);
//
//                  find_then!(events, GameEvent::DealerScored { player, pegging } => {
//                      assert_eq!(player, &PLAYER1);
//                      assert_eq!(pegging.recipient(), &PLAYER0);
//                      assert_eq!(pegging.score_sheet().points(), Points::from(4));
//                  });
//              },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Finished(finished) => {
//                      assert_eq!(finished.winner(), PLAYER0);
//                  });
//              }
//          }
//      }
//
//      #[test]
//      fn score_crib_when_dealer_score_acknowledged() {
//          game_test! {
//              given: &scenario!(
//                  build_scoring_dealer;
//                  with_points(0, 0),
//                  with_cut("4H"),
//                  with_hands("7H8CAC2C", "JCKS5HTH"),
//                  with_crib("AHADASTD"),
//                  with_ack(0),
//              ),
//              when: GameCommand::ScoreCrib { player: PLAYER1 },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events.len(), 1);
//
//                  find_then!(events, GameEvent::CribScored { player, pegging } => {
//                      assert_eq!(player, &PLAYER1);
//                      assert_eq!(pegging.recipient(), &PLAYER0);
//                      assert_eq!(pegging.score_sheet().points(), Points::from(12));
//                  });
//              }
//          }
//      }
//
//      #[test]
//      fn score_winning_crib_when_dealer_score_acknowledged() {
//          game_test! {
//              given: &scenario!(
//                  build_scoring_dealer;
//                  with_points(109, 0),
//                  with_cut("4H"),
//                  with_hands("7H8CAC2C", "JCKS5HTH"),
//                  with_crib("AHADASTD"),
//                  with_ack(0),
//              ),
//              when: GameCommand::ScoreCrib { player: PLAYER1 },
//              then_events: |events: &[GameEvent]| {
//                  assert_eq!(events.len(), 1);
//
//                  find_then!(events, GameEvent::CribScored { player, pegging } => {
//                      assert_eq!(player, &PLAYER1);
//                      assert_eq!(pegging.recipient(), &PLAYER0);
//                      assert_eq!(pegging.score_sheet().points(), Points::from(12));
//                  });
//              },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Finished(finished) => {
//                      assert_eq!(finished.winner(), PLAYER0);
//                  });
//              }
//          }
//      }
//
//      #[test]
//      fn redeal_when_crib_score_acknowledged() {
//          game_test! {
//              given: &scenario!(
//                  build_scoring_crib;
//                  with_points(0, 0),
//                  with_cut("4H"),
//                  with_hands("7H8CAC2C", "JCKS5HTH"),
//                  with_crib("AHADASTD"),
//                  with_ack(0)
//              ),
//              when: GameCommand::StartNextRound { player: PLAYER1 },
//              then_events: |events: &[GameEvent]| {
//                  find_then!(events, GameEvent::NextRoundStarted { player } => {
//                      assert_eq!(player, &PLAYER1);
//                  });
//                  let deals = events
//                      .iter()
//                      .filter(|e| matches!(e, GameEvent::HandDealt { .. }))
//                      .collect::<Vec<_>>();
//                  assert_eq!(deals.len(), PLAYER_COUNT);
//              },
//              then_phase: |phase: &Phase| {
//                  assert_phase_then!(phase, Phase::Discarding(discarding) => {
//                      assert_eq!(discarding.dealer(), &Dealer::from(PLAYER1));
//                      assert_eq!(discarding.pone(), &Pone::from(PLAYER0));
//                      assert_eq!(discarding.hand(PLAYER0).len(), CARDS_DEALT_PER_HAND);
//                      assert_eq!(discarding.hand(PLAYER1).len(), CARDS_DEALT_PER_HAND);
//                  })
//              }
//          }
//      }
//
//      #[test]
//      fn hand_should_score_fifteens() {
//          assert_eq!(
//              ScoreSheet::hand(&hand!("7H8CAC2C"), card!("4H")).points(),
//              Points::from(4)
//          );
//          assert_eq!(
//              ScoreSheet::hand(&hand!("THJCKS5H"), card!("4H")).points(),
//              Points::from(6)
//          );
//      }
//
//      #[test]
//      fn hand_should_score_pairs() {
//          assert_eq!(
//              ScoreSheet::hand(&hand!("2H4C5C2C"), card!("AH")).points(),
//              Points::from(2)
//          );
//          assert_eq!(
//              ScoreSheet::hand(&hand!("TCASADTH"), card!("AH")).points(),
//              Points::from(8)
//          );
//      }
//
//      #[test]
//      fn hand_should_score_royal_pairs() {
//          assert_eq!(
//              ScoreSheet::hand(&hand!("2H2D5C2C"), card!("AH")).points(),
//              Points::from(6)
//          );
//          assert_eq!(
//              ScoreSheet::hand(&hand!("TCASADTH"), card!("AH")).points(),
//              Points::from(8)
//          );
//      }
//
//      #[test]
//      fn hand_should_score_double_royal_pairs() {
//          assert_eq!(
//              ScoreSheet::hand(&hand!("2H2C2D2S"), card!("AH")).points(),
//              Points::from(12)
//          );
//          assert_eq!(
//              ScoreSheet::hand(&hand!("TCASADTH"), card!("AH")).points(),
//              Points::from(8)
//          );
//      }
//
//      #[test]
//      fn hand_should_score_runs() {
//          assert_eq!(
//              ScoreSheet::hand(&hand!("JDQCKC2C"), card!("AH")).points(),
//              Points::from(3)
//          );
//          assert_eq!(
//              ScoreSheet::hand(&hand!("3C3S2D5H"), card!("AH")).points(),
//              Points::from(8)
//          );
//      }
//
//      #[test]
//      fn hand_should_score_flushes() {
//          assert_eq!(
//              ScoreSheet::hand(&hand!("2H4H6H8H"), card!("TH")).points(),
//              Points::from(5)
//          );
//          assert_eq!(
//              ScoreSheet::hand(&hand!("2D4D6D8D"), card!("TH")).points(),
//              Points::from(4)
//          );
//      }
//
//      #[test]
//      fn hand_should_score_nobs() {
//          assert_eq!(
//              ScoreSheet::hand(&hand!("2D4H6HJH"), card!("TH")).points(),
//              Points::from(1)
//          );
//          assert_eq!(
//              ScoreSheet::hand(&hand!("2H4D6DJD"), card!("TH")).points(),
//              Points::from(0)
//          );
//      }
//
//      #[test]
//      fn crib_should_score_fifteens() {
//          assert_eq!(
//              ScoreSheet::crib(&crib!("7H8CAC2C"), card!("4H")).points(),
//              Points::from(4)
//          );
//          assert_eq!(
//              ScoreSheet::crib(&crib!("THJCKS5H"), card!("4H")).points(),
//              Points::from(6)
//          );
//      }
//
//      #[test]
//      fn crib_should_score_pairs() {
//          assert_eq!(
//              ScoreSheet::crib(&crib!("2H4C5C2C"), card!("AH")).points(),
//              Points::from(2)
//          );
//          assert_eq!(
//              ScoreSheet::crib(&crib!("TCASADTH"), card!("AH")).points(),
//              Points::from(8)
//          );
//      }
//
//      #[test]
//      fn crib_should_score_royal_pairs() {
//          assert_eq!(
//              ScoreSheet::crib(&crib!("2H2D5C2C"), card!("AH")).points(),
//              Points::from(6)
//          );
//          assert_eq!(
//              ScoreSheet::crib(&crib!("TCASADTH"), card!("AH")).points(),
//              Points::from(8)
//          );
//      }
//
//      #[test]
//      fn crib_should_score_double_royal_pairs() {
//          assert_eq!(
//              ScoreSheet::crib(&crib!("2H2C2D2S"), card!("AH")).points(),
//              Points::from(12)
//          );
//          assert_eq!(
//              ScoreSheet::crib(&crib!("TCASADTH"), card!("AH")).points(),
//              Points::from(8)
//          );
//      }
//
//      #[test]
//      fn crib_should_score_runs() {
//          assert_eq!(
//              ScoreSheet::crib(&crib!("JDQCKC2C"), card!("AH")).points(),
//              Points::from(3)
//          );
//          assert_eq!(
//              ScoreSheet::crib(&crib!("3C3S2D5H"), card!("AH")).points(),
//              Points::from(8)
//          );
//      }
//
//      #[test]
//      fn crib_should_score_flushes() {
//          assert_eq!(
//              ScoreSheet::crib(&crib!("2H4H6H8H"), card!("TH")).points(),
//              Points::from(5)
//          );
//          assert_eq!(
//              ScoreSheet::crib(&crib!("2D4D6D8D"), card!("TH")).points(),
//              Points::from(0)
//          );
//      }
//
//      #[test]
//      fn crib_should_score_nobs() {
//          assert_eq!(
//              ScoreSheet::crib(&crib!("2D4H6HJH"), card!("TH")).points(),
//              Points::from(1)
//          );
//          assert_eq!(
//              ScoreSheet::crib(&crib!("2H4D6DJD"), card!("TH")).points(),
//              Points::from(0)
//          );
//      }
//  }
//
//  /// ### Combinations
//  ///
//  /// In the above table, the word combination is used in the strict technical sense. Each and
//  /// every combination of two cards that make a pair, of two or more cards that make 15, or of
//  /// three or more cards that make a run, count separately.
//  ///
//  /// Example: A hand (including the starter) comprised of 8, 7, 7, 6, 2 scores 8 points for four
//  /// combinations that total 15: the 8 with one 7, and the 8 with the other 7; the 6, 2 with each
//  /// of the two 7s. The same hand also scores 2 for a pair, and 6 for two runs of three (8, 7, 6
//  /// using each of the two 7s). The total score is 16. An experienced player computes the hand
//  /// thus: "Fifteen 2, fifteen 4, fifteen 6, fifteen 8, and 8 for double run is 16."
//  ///
//  /// Note that the ace is always low and cannot form a sequence with a king. Further, a flush
//  /// cannot happen during the play of the cards; it occurs only when the hands and the crib are
//  /// counted.
//  ///
//  /// Certain basic formulations should be learned to facilitate counting. For pairs and runs
//  /// alone:
//  ///
//  /// A. A triplet counts 6. A. Four of a kind counts 12. A. A run of three, with one card
//  /// duplicated (double run) counts 8. A. A run of four, with one card duplicated, counts 10. A.
//  /// A run of three, with one card triplicated (triple run), counts 15. A. A run of three, with
//  /// two different cards duplicated, counts 16.
//  #[allow(clippy::expect_used)]
//  mod combinations {
//      use super::*;
//
//      #[test]
//      fn should_score_rules_example_eights_sevens_sixes() {
//          assert_eq!(
//              ScoreSheet::hand(&hand!("8H7C7D6S"), card!("2H")).points(),
//              Points::from(16)
//          );
//      }
//
//      #[test]
//      fn should_score_rules_example_runs() {
//          assert_eq!(
//              ScoreSheet::hand(&hand!("JHQCKDAS"), card!("2D")).points(),
//              Points::from(3)
//          );
//      }
//
//      #[test]
//      fn should_score_rules_example_flush() {
//          assert_eq!(
//              ScoreSheet::hand(&hand!("THQHKHAH"), card!("2H")).points(),
//              Points::from(5)
//          );
//          assert_eq!(
//              ScoreSheet::hand(&hand!("THQHKHAH"), card!("2S")).points(),
//              Points::from(4)
//          );
//          assert_eq!(
//              ScoreSheet::hand(&hand!("THQHKHAS"), card!("2H")).points(),
//              Points::from(0)
//          );
//      }
//  }
//
//  /// ### A PERFECT 29!
//  ///
//  /// The highest possible score for combinations in a single Cribbage deal is 29, and it may
//  /// occur only once in a Cribbage fan's lifetime -in fact, experts say that a 29 is probably as
//  /// rare as a hole-in-one in golf. To make this amazing score, a player must have a five as the
//  /// starter (upcard) and the other three fives plus the jack of the same suit as the starter -
//  /// His Nobs: 1 point - in his hand. The double pair royal (four 5s) peg another 12 points; the
//  /// various fives used to hit 15 can be done four ways for 8 points; and the jack plus a 5 to
//  /// hit 15 can also be done four ways for 8 points. Total = 29 points.
//  #[allow(clippy::expect_used)]
//  mod a_perfect_29 {
//      use super::*;
//
//      #[test]
//      fn should_score_rules_example_perfect_29() {
//          assert_eq!(
//              ScoreSheet::hand(&hand!("5H5C5DJS"), card!("5S")).points(),
//              Points::from(29)
//          );
//      }
//  }
//
//  /// ## Miscellaneous
//  ///
//  /// The following list includes many of the hands that may give the beginner some difficulty in
//  /// counting. Note that no hand can make a count of 19, 25, 26, or 27. (In the chart below J
//  /// stands for His Nobs, the jack of the same suit as the starter.
//  ///
//  /// ### Muggins (optional) - not implemented.
//  ///
//  /// Each player must count his hand (and crib) aloud and announce the total. If he overlooks any
//  /// score, the opponent may say "Muggins" and then score the overlooked points for himself. For
//  /// experienced players, the Muggins rule is always in effect and adds even more suspense to the
//  /// game.
//  #[allow(clippy::expect_used)]
//  mod miscellaneous {}
//
//  /// ## Game
//  ///
//  /// Game may be fixed at either 121 points or 61 points. The play ends the moment either player
//  /// reaches the agreed total, whether by pegging or counting one's hand. If the non-dealer "goes
//  /// out" by the count of his hand, the game immediately ends and the dealer may not score either
//  /// his hand or the crib.
//  ///
//  /// If a player wins the game before the loser has passed the halfway mark (did not reach 31 in
//  /// a game of 61, or 61 in a game of 121), the loser is "lurched," and the winner scores two
//  /// games instead of one. A popular variation of games played to 121, is a "skunk" (double game)
//  /// for the winner if the losing player fails to pass the three-quarter mark - 91 points or more -
//  /// and it is a "double skunk" (quadruple game) if the loser fails to pass the halfway mark (61
//  /// or more points).
//  #[allow(clippy::expect_used)]
//  mod game {}
//
//  /// ## The Cribbage Board
//  ///
//  /// The Cribbage board (see illustration) has four rows of 30 holes each, divided into two pairs
//  /// of rows by a central panel. There are usually four (or two) additional holes near one end,
//  /// called "game holes." With the board come four pegs, usually in two contrasting colors. Note:
//  /// There are also continuous track Cribbage boards available which, as the name implies, have
//  /// one continuous line of 121 holes for each player.
//  ///
//  /// The board is placed to one side between the two players, and each player takes two pegs of
//  /// the same color. (The pegs are placed in the game holes until the game begins.) Each time a
//  /// player scores, he advances a peg along a row on his side of the board, counting one hole per
//  /// point. Two pegs are used, and the rearmost peg jumps over the first peg to show the first
//  /// increment in score. After another increase in score, the peg behind jumps over the peg in
//  /// front to the appropriate hole to show the player's new score, and so on (see diagram next
//  /// page). The custom is to "go down" (away from the game holes) on the outer rows and "come up"
//  /// on the inner rows. A game of 61 is "once around" and a game of 121 is "twice around." As
//  /// noted previously, continuous line Cribbage boards are available.
//  ///
//  /// If a Cribbage board is not available, each player may use a piece of paper or cardboard,
//  /// marked thus:
//  ///
//  ///   - Units 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
//  ///   - Tens 10, 20, 30, 40, 50, 60
//  ///
//  /// Two small markers, such as small coins or buttons, can substitute for pegs for counting in
//  /// each row.
//  #[allow(clippy::expect_used)]
//  mod the_cribbage_board {}
//
//  /// ## Strategy
//  ///
//  /// ### The Crib.
//  ///
//  /// If the dealer is discarding for the crib, he should “salt” it with the best possible cards,
//  /// but at the same time retain good cards in his hand that can be used for high scoring.
//  /// Conversely, for the non-dealer, it is best to lay out cards that will be the least
//  /// advantageous for the dealer. Laying out a five would be the worst choice, for the dealer
//  /// could use it to make 15 with any one of the ten-cards (10, J, Q, K). Laying out a pair is
//  /// usually a poor choice too, and the same goes for sequential cards, such as putting both a
//  /// six and seven in the crib. The ace and king tend to be good cards to put in the crib because
//  /// it is harder to use them in a run.
//  ///
//  /// ### The Play
//  ///
//  /// As expected, the five makes for the worst lead in that there are so many ten-cards that the
//  /// opponent can use to make a 15. Leading from a pair is a good idea, for even if the opponent
//  /// makes a pair, the leader can play the other matching card from his hand and collect for a
//  /// pair royal. Leading an ace or deuce is not a good idea, for these cards should be saved
//  /// until later to help make a 15, a Go, or a 31. The safest lead is a four because this card
//  /// cannot be used to make a 15 at the opponent’s very next turn. Finally, when the opponent
//  /// leads a card that can either be paired or make 15, the latter choice is preferred.
//  ///
//  /// During the play, it is advisable not to try to make a count of 21, for the opponent can then
//  /// play one of the many 10-cards and make 31 to gain two points.
//  #[allow(clippy::expect_used)]
//  mod the_strategy {}
//
//  /// ## Internal
//  #[allow(clippy::expect_used)]
//  mod internal {
//      use super::*;
//
//      fn common_filters() -> insta::Settings {
//          let mut settings = insta::Settings::new();
//          settings.add_filter(
//              r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{1,9}",
//              "<timestamp>",
//          );
//          settings.add_filter(
//              r"UserId\([0-9a-f]{8}-([0-9a-f]{4}-){3}[0-9a-f]{12}\)",
//              "<userid>",
//          );
//          settings.add_filter(r"Player\([0-1]\)", "<player>");
//          settings.add_filter(r"(A|[2-9]|T|J|Q|K)(H|C|D|S)", "<card>");
//          settings.add_filter(r"<card>(, <card>)*", "[<cards>]");
//          settings.add_filter(r"\s*\d+ ->\s*\d+", "<score>");
//          settings
//      }
//
//      #[test]
//      fn should_output_user_readable_starting_game_in_logs() {
//          let game = GameBuilder::default().with_cuts("ASAC").build_starting();
//          common_filters().bind(|| {
//              insta::assert_snapshot!(game.to_string(), @r"
//                  test-game__<timestamp> U[<cards>]
//                  <userid> <userid>
//                  Starting(
//                      cuts: [<cards>]
//                      deck: Deck([<cards>])
//                      pending: Pending(<player>, <player>)
//                  )
//                  ")
//          });
//      }
//
//      #[test]
//      fn should_output_user_readable_discarding_game_in_logs() {
//          let game = GameBuilder::default()
//              .with_points(0, 0)
//              .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
//              .build_discarding();
//          common_filters().bind(|| {
//              insta::assert_snapshot!(game.to_string(), @r"
//                  test-game__<timestamp> U[<cards>]
//                  <userid> <userid>
//                  Discarding(
//                      scoreboard: Scoreboard(<score>,<score>)
//                      roles: Dealer(<player>), Pone(<player>)
//                      hands: Hand([<cards>]), Hand([<cards>])
//                      crib: Crib()
//                      deck: Deck([<cards>])
//                      pending: Pending(<player>, <player>)
//                  )
//                  ")
//          });
//      }
//
//      #[test]
//      fn should_output_user_readable_playing_game_in_logs() {
//          let game = GameBuilder::default()
//              .with_points(0, 0)
//              .with_hands("9S", "4S")
//              .with_cut("AS")
//              .with_current_plays(&[(0, "AH")])
//              .build_playing(1);
//          common_filters().bind(|| insta::assert_snapshot!(game.to_string(), @r"
//                                       test-game__<timestamp> U[<cards>]
//                                       <userid> <userid>
//                                       Playing(
//                                           scoreboard: Scoreboard(<score>,<score>),
//                                           roles: Dealer(<player>), Pone(<player>),
//                                           hands: Hand([<cards>]), Hand([<cards>]),
//                                           play_state: Next(<player>), GoStatus(NotCalled), Pending(<player> -> [<cards>], <player> -> [<cards>]), Current((<player> -> [<cards>])), Previous(),
//                                           cut: [<cards>],
//                                           crib: Crib(),
//                                           pending: Pending(<player>, <player>)
//                                       )
//                                       "));
//      }
//
//      #[test]
//      fn should_output_user_readable_pone_scoring_game_in_logs() {
//          let game = GameBuilder::default()
//              .with_points(0, 0)
//              .with_hands("AS2S3S4S", "AC2C3C4C")
//              .with_cut("JH")
//              .with_crib("TSJSQSKS")
//              .build_scoring_pone();
//          common_filters().bind(|| {
//                  insta::assert_snapshot!(game.to_string(), @r"
//                  test-game__<timestamp> U[<cards>]
//                  <userid> <userid>
//                  ScoringPone(
//                      scoreboard: Scoreboard(<score>,<score>),
//                      roles: Dealer(<player>), Pone(<player>),
//                      hands: Hand([<cards>]), Hand([<cards>]),
//                      cut: [<cards>],
//                      crib: Crib([<cards>]),
//                      pegging: <player> -> Fifteen: ([<cards>]) -> 2, Fifteen: ([<cards>]) -> 2, Run: ([<cards>]) -> 4, Flush: ([<cards>]) -> 4,
//                      pending: Pending(<player>, <player>)
//                  )
//                  ")
//              });
//      }
//
//      #[test]
//      fn should_output_user_readable_dealer_scoring_game_in_logs() {
//          let game = GameBuilder::default()
//              .with_points(0, 0)
//              .with_hands("AS2S3S4S", "AC2C3C4C")
//              .with_cut("JH")
//              .with_crib("TSJSQSKS")
//              .build_scoring_dealer();
//          common_filters().bind(|| {
//                  insta::assert_snapshot!(game.to_string(), @r"
//                  test-game__<timestamp> U[<cards>]
//                  <userid> <userid>
//                  ScoringDealer(
//                      scoreboard: Scoreboard(<score>,<score>),
//                      roles: Dealer(<player>), Pone(<player>),
//                      hands: Hand([<cards>]), Hand([<cards>]),
//                      cut: [<cards>],
//                      crib: Crib([<cards>]),
//                      pegging: <player> -> Fifteen: ([<cards>]) -> 2, Fifteen: ([<cards>]) -> 2, Run: ([<cards>]) -> 4, Flush: ([<cards>]) -> 4,
//                      pending: Pending(<player>, <player>)
//                  )
//                  ")
//              });
//      }
//
//      #[test]
//      fn should_output_user_readable_crib_scoring_game_in_logs() {
//          let game = GameBuilder::default()
//              .with_points(0, 0)
//              .with_hands("AS2S3S4S", "AC2C3C4C")
//              .with_cut("JH")
//              .with_crib("TSJSQSKS")
//              .build_scoring_crib();
//          common_filters().bind(|| {
//                  insta::assert_snapshot!(game.to_string(), @r"
//                  test-game__<timestamp> U[<cards>]
//                  <userid> <userid>
//                  ScoringCrib(
//                      scoreboard: Scoreboard(<score>,<score>),
//                      roles: Dealer(<player>), Pone(<player>),
//                      hands: Hand([<cards>]), Hand([<cards>]),
//                      cut: [<cards>],
//                      crib: Crib([<cards>]),
//                      pegging: <player> -> Pair: ([<cards>]) -> 2, Run: ([<cards>]) -> 4, Run: ([<cards>]) -> 4,
//                      pending: Pending(<player>, <player>)
//                  )
//                  ")
//              });
//      }
//
//      #[test]
//      fn should_output_user_readable_finished_game_in_logs() {
//          let game = GameBuilder::default()
//              .with_points(0, 121)
//              .with_winner(1)
//              .with_hands("AS2S3S4S", "AC2C3C4C")
//              .with_cut("JH")
//              .with_crib("TSJSQSKS")
//              .build_finished();
//          common_filters().bind(|| {
//              insta::assert_snapshot!(game.to_string(), @r"
//                  test-game__<timestamp> U[<cards>]
//                  <userid> <userid>
//                  Finished(
//                      winner: <player>,
//                      scoreboard: Scoreboard(<score>,<score>),
//                      roles: Dealer(<player>), Pone(<player>),
//                      hands: Hand([<cards>]), Hand([<cards>]),
//                      crib: Crib([<cards>]),
//                      cut: [<cards>]
//                  )
//                  ")
//          });
//      }
//  }
//
