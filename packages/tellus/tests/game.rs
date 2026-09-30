use cribbage_domain::prelude::*;
use tellus::{ActorRef, ActorSystem};
use ulid::Ulid;

use crate::{CreateGame, GameId, Games, JoinGame, Player, UserId};

#[tokio::test]
async fn user_creates_game_and_user2_joins() {
    let system = ActorSystem::new(Games::default());
    let games = system.root();

    let user1 = UserId::new();
    let user2 = UserId::new();

    let game_id = games
        .ask(std::time::Duration::from_secs(1), |reply| CreateGame {
            user_id: user1,
            deck: Deck::new(),
            reply,
        })
        .await
        .unwrap();

    let (game_id, player) = games
        .ask(std::time::Duration::from_secs(1), |reply| CreateGame {
            user_id: user1,
            deck: Deck::new(),
            reply,
        })
        .await
        .unwrap();

    assert_eq!(player, Player::Player0);
    assert_eq!(player, Player::Player1);
}
