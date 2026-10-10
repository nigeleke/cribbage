mod test_support;

// ------------------------------------
use cribbage_domain::prelude::Player;
use cribbage_tellus::prelude::{DeckSource, GameCommand, GameEvent, PersistedCards, UserId, Users};
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
            deck: deck.clone(),
        })
        .then(scenario.state());
}

#[test]
fn user1_cuts_for_deal() {
    let scenario = GameScenario::default().progress_to_starting();
    let users = scenario.users();
    let host = users.user(Player::Player0);

    scenario
        .to_fixture()
        .when(GameCommand::CutForDeal { user: host })
        .then(scenario.cut_for_deal(host).state());
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
        .then(GameScenario::default().state());
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
        .then(GameScenario::default().state());
}

#[test]
fn hands_dealt_after_cut_made() {
    let scenario = GameScenario::default().progress_to_dealing();
    let deck = ShuffledDeckSource::new().new_deck();
    let deck = PersistedCards::from(deck.as_ref());

    scenario
        .to_fixture()
        .when(GameCommand::DealHands { deck: deck.clone() })
        .then(GameScenario::default().state());
}

// #[test]// async fn user1_discards_to_crib() {
//     let scenario = GameScenario::default().progress_to_discarding();
//     let users = scenario.users();
//     let user = users.user(Player::Player0);
//     let hand = scenario.hand(Player::Player0).to_vec();
//     let discard = [hand[0], hand[1]]
//         .into_iter()
//         .map(|c| PersistedCard::from(*c));
//
//     scenario
//         .to_fixture()
//         .await
//         .when_tell(GameCommand::Discard { user, discard })
// }

#[test]
fn user_discards_to_crib_twice() {
    todo!()
}

#[test]
fn user2_discards_to_crib() {
    todo!()
}

#[test]
fn user_plays_card() {
    todo!()
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
