mod test_support;

use std::assert_matches;

// ------------------------------------
use cribbage_domain::prelude::{HasHands, HasPlayState, HasRoles, Player};
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
    let scenario = GameScenario::default().given_starting();
    let users = scenario.users();
    let host = users.user(Player::Player0);

    scenario
        .to_fixture()
        .when(GameCommand::CutForDeal { user: host })
        .then(|actual| assert_matches!(actual, GameState::Starting { .. }));
}

#[test]
fn user_cuts_for_deal_twice() {
    let scenario = GameScenario::default().given_starting();
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
    let scenario = GameScenario::default().given_starting();
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
    let scenario = GameScenario::default().given_dealing();
    let deck = ShuffledDeckSource::new().new_deck();
    let deck = PersistedCards::from(deck.as_ref());

    scenario
        .to_fixture()
        .when(GameCommand::DealHands { deck })
        .then(|actual| assert_matches!(actual, GameState::Discarding { .. }));
}

#[test]
fn user1_discards_to_crib() {
    let scenario = GameScenario::default().given_discarding();
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
    let scenario = GameScenario::default().given_discarding();
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
    let scenario = GameScenario::default().given_discarding();
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
    let scenario = GameScenario::default().given_cutting();

    scenario
        .to_fixture()
        .when(GameCommand::CutStarter)
        .then(|actual| assert_matches!(actual, GameState::Playing { .. }));
}

#[test]
fn user_plays_card() {
    let scenario = GameScenario::default().given_playing();
    let player = scenario.pone();
    let user = scenario.users().user(player);
    let hand = scenario.hand(player).to_vec();
    let card = PersistedCard::from(&hand[0]);

    scenario
        .to_fixture()
        .when(GameCommand::PlayCard { user, card })
        .then(|actual| assert_matches!(actual, GameState::Playing { .. }));
}

#[test]
fn user_go() {
    let scenario = GameScenario::default().given_go();
    let player = scenario.next_to_play();
    let user = scenario.users().user(player);

    scenario
        .to_fixture()
        .when(GameCommand::Go { user })
        .then(|actual| assert_matches!(actual, GameState::Playing { .. }));
}

#[test]
fn pone_is_scored() {
    let scenario = GameScenario::default().given_scoring_pone();

    scenario
        .to_fixture()
        .when(GameCommand::ScorePone)
        .then(|actual| assert_matches!(actual, GameState::ScoringDealer { .. }));
}

#[test]
fn dealer_is_scored() {
    let scenario = GameScenario::default().given_scoring_dealer();

    scenario
        .to_fixture()
        .when(GameCommand::ScoreDealer)
        .then(|actual| assert_matches!(actual, GameState::ScoringCrib { .. }));
}

#[test]
fn crib_is_scored() {
    let scenario = GameScenario::default().given_scoring_crib();

    scenario
        .to_fixture()
        .when(GameCommand::ScoreCrib)
        .then(|actual| assert_matches!(actual, GameState::Dealing { .. }));
}

#[test]
fn game_is_won() {
    let (scenario, winning_command) = GameScenario::default().given_finished();
    let events = scenario.events();
    let events = &events[..events.len().saturating_sub(1)];

    let fixture = scenario.to_fixture_init().given(events);
    assert!(!matches!(fixture.state(), GameState::Finished { .. }));

    fixture
        .when(winning_command)
        .then(|actual| assert_matches!(actual, GameState::Finished { .. }));
}
