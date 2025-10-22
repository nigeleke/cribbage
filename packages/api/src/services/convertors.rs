use dto::UserGameDTO;

pub fn game_to_user_game_dto(game: &backend::Game, user_id: &backend::UserId) -> UserGameDTO {
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
