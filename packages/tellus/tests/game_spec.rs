mod test_support;

// ------------------------------------
use cribbage_domain::prelude::Player;
use cribbage_tellus::prelude::{
    DeckSource, Game, GameCommand, GameEvent, GameId, PersistedCard, PersistedCards, UserId, Users,
};
use test_support::{EntityFixture, GameScenario, ShuffledDeckSource};

#[tokio::test]
async fn game_is_created() {
    let scenario = GameScenario::default();
    let users = Users::new(UserId::new(), UserId::new());
    let deck = ShuffledDeckSource::new().new_deck();
    let deck = PersistedCards::from(deck.as_ref());

    scenario
        .to_fixture()
        .await
        .when_tell(GameCommand::CreateGame {
            users: users,
            deck: deck.clone(),
        })
        .await
        .then_events(&[GameEvent::GameCreated { users, deck }]);
}

#[tokio::test]
async fn user1_cuts_for_deal() {
    let scenario = GameScenario::default().progress_to_starting();
    let users = scenario.users();
    let host = users.user(Player::Player0);

    scenario
        .to_fixture()
        .await
        .when_tell(GameCommand::CutForDeal { user: host })
        .await
        .then_events(&[GameEvent::CutForDealMade { user: host }]);
}

#[tokio::test]
async fn user_cuts_for_deal_twice() {
    let scenario = GameScenario::default().progress_to_starting();
    let users = scenario.users();
    let host = users.user(Player::Player0);

    scenario
        .to_fixture()
        .await
        .given_event(&GameEvent::CutForDealMade { user: host })
        .await
        .when_tell(GameCommand::CutForDeal { user: host })
        .await
        .then_events(&[]);
}

#[tokio::test]
async fn user2_cuts_for_deal() {
    let scenario = GameScenario::default().progress_to_starting();
    let users = scenario.users();
    let host = users.user(Player::Player0);
    let guest = users.user(Player::Player1);

    scenario
        .to_fixture()
        .await
        .given_event(&GameEvent::CutForDealMade { user: host })
        .await
        .when_tell(GameCommand::CutForDeal { user: guest })
        .await
        .then_events(&[GameEvent::CutForDealMade { user: guest }]);
}

#[tokio::test]
async fn hands_dealt_after_cut_made() {
    let scenario = GameScenario::default().progress_to_dealing();
    let deck = ShuffledDeckSource::new().new_deck();
    let deck = PersistedCards::from(deck.as_ref());

    scenario
        .to_fixture()
        .await
        .when_tell(GameCommand::DealHands { deck: deck.clone() })
        .await
        .then_events(&[GameEvent::HandsDealt { deck: deck }]);
}

// #[tokio::test]
// async fn user1_discards_to_crib() {
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

#[tokio::test]
async fn user_discards_to_crib_twice() {
    todo!()
}

#[tokio::test]
async fn user2_discards_to_crib() {
    todo!()
}

#[tokio::test]
async fn user_plays_card() {
    todo!()
}

#[tokio::test]
async fn user_go() {
    todo!()
}

#[tokio::test]
async fn pone_is_scored() {
    todo!()
}

#[tokio::test]
async fn dealer_is_scored() {
    todo!()
}
