mod test_support;

// ------------------------------------
use cribbage_domain::prelude::Player;
use cribbage_tellus::prelude::{
    DeckSource, Game, GameCommand, GameEvent, GameId, PersistedCards, UserId, Users,
};
use tellus::EventSourced;
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
