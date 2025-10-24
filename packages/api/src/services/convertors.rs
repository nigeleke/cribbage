use backend::{AvailableGame, AvailableGameSource, Game};
use dto::{AvailableGameDTO, GameIdDTO, UserGameDTO};

pub fn game_to_user_game_dto(game: &Game, user_id: &backend::UserId) -> UserGameDTO {
    let user_is_host = || game.host() == user_id;
    let user_is_guest = || game.guest() == Some(user_id);

    if user_is_host() {
        UserGameDTO::new(game.name())
    } else if user_is_guest() {
        UserGameDTO::new(game.name())
    } else {
        UserGameDTO::new(game.name())
    }
}

pub fn available_game_to_dto(game: &AvailableGame) -> AvailableGameDTO {
    let game_id = GameIdDTO::from(game.id().value());
    let name = game.name().clone();
    let source = game.source();
    match source {
        AvailableGameSource::Lobby => AvailableGameDTO::Lobby { game_id, name },
        AvailableGameSource::Active => AvailableGameDTO::Active { game_id, name },
    }
}
