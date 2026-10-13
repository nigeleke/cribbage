mod test_support;

use std::assert_matches;

// ------------------------------------
use cribbage_domain::prelude::Player;
use cribbage_tellus::prelude::{
    DeckSource, GameCommand, GameError, GameEvent, GameState, PersistedCard, PersistedCards,
    PersistedDiscard, UserId, Users,
};
use test_support::{GameScenario, ShuffledDeckSource};

#[test]
fn game_is_created() {
    let scenario = GameScenario::default();
    let users = Users::new(UserId::new(), UserId::new());
    let deck = ShuffledDeckSource::new().new_deck();
    let deck = PersistedCards::from(deck.as_ref());

    scenario
        .to_fixture()
        .when(GameCommand::CreateGame {
            users: users,
            deck: deck,
        })
        .then(|actual| assert_matches!(actual, GameState::Starting { .. }));
}

#[test]
fn user1_cuts_for_deal() {
    let scenario = GameScenario::default().progress_to_starting();
    let users = scenario.users();
    let host = users.user(Player::Player0);

    scenario
        .to_fixture()
        .when(GameCommand::CutForDeal { user: host })
        .then(|actual| assert_matches!(actual, GameState::Starting { .. }));
}

#[test]
fn user_cuts_for_deal_twice() {
    let scenario = GameScenario::default().progress_to_starting();
    let users = scenario.users();
    let host = users.user(Player::Player0);

    scenario
        .to_fixture()
        .given_event(GameEvent::CutForDealMade { user: host })
        .when(GameCommand::CutForDeal { user: host })
        .then_error(|error| assert_matches!(error, GameError::DomainError(_)));
}

#[test]
fn user2_cuts_for_deal() {
    let scenario = GameScenario::default().progress_to_starting();
    let users = scenario.users();
    let host = users.user(Player::Player0);
    let guest = users.user(Player::Player1);

    scenario
        .to_fixture()
        .given_event(GameEvent::CutForDealMade { user: host })
        .when(GameCommand::CutForDeal { user: guest })
        .then(|actual| assert_matches!(actual, GameState::Dealing { .. }));
}

#[test]
fn hands_dealt_after_cut_made() {
    let scenario = GameScenario::default().progress_to_dealing();
    let deck = ShuffledDeckSource::new().new_deck();
    let deck = PersistedCards::from(deck.as_ref());

    scenario
        .to_fixture()
        .when(GameCommand::DealHands { deck })
        .then(|actual| assert_matches!(actual, GameState::Discarding { .. }));
}

#[test]
fn user1_discards_to_crib() {
    let scenario = GameScenario::default().progress_to_discarding();
    let users = scenario.users();
    let user = users.user(Player::Player0);
    let hand = scenario.hand(Player::Player0).to_vec();
    let discard = PersistedDiscard::from(&[hand[0], hand[1]]);

    scenario
        .to_fixture()
        .when(GameCommand::Discard { user, discard })
        .then(|actual| assert_matches!(actual, GameState::Discarding { .. }));
}

#[test]
fn user_discards_to_crib_twice() {
    let scenario = GameScenario::default().progress_to_discarding();
    let users = scenario.users();
    let user = users.user(Player::Player0);
    let hand = scenario.hand(Player::Player0).to_vec();
    let discard = PersistedDiscard::from(&[hand[0], hand[1]]);

    scenario
        .to_fixture()
        .given_event(GameEvent::DiscardMade { user, discard })
        .when(GameCommand::Discard { user, discard })
        .then_error(|error| assert_matches!(error, GameError::DomainError(_)));
}

#[test]
fn user2_discards_to_crib() {
    let scenario = GameScenario::default().progress_to_discarding();
    let users = scenario.users();

    let user0 = users.user(Player::Player0);
    let hand0 = scenario.hand(Player::Player0).to_vec();
    let discard0 = PersistedDiscard::from(&[hand0[0], hand0[1]]);

    let user1 = users.user(Player::Player1);
    let hand1 = scenario.hand(Player::Player1).to_vec();
    let discard1 = PersistedDiscard::from(&[hand1[0], hand1[1]]);

    scenario
        .to_fixture()
        .given_event(GameEvent::DiscardMade {
            user: user0,
            discard: discard0,
        })
        .when(GameCommand::Discard {
            user: user1,
            discard: discard1,
        })
        .then(|actual| assert_matches!(actual, GameState::Cutting { .. }));
}

#[test]
fn starter_card_is_cut() {
    let scenario = GameScenario::default().progress_to_cutting();

    scenario
        .to_fixture()
        .when(GameCommand::CutStarter)
        .then(|actual| assert_matches!(actual, GameState::Playing { .. }));
}

#[test]
fn user_plays_card() {
    let scenario = GameScenario::default().progress_to_playing();
    let users = scenario.users();

    let user = users.user(Player::Player0);
    let hand = scenario.hand(Player::Player0).to_vec();
    let card = PersistedCard::from(&hand[0]);

    scenario
        .to_fixture()
        .when(GameCommand::PlayCard { user, card })
        .then(|actual| assert_matches!(actual, GameState::Playing { .. }));
}

#[test]
fn user_go() {
    todo!()
}

#[test]
fn pone_is_scored() {
    todo!()
}

#[test]
fn dealer_is_scored() {
    todo!()
}
