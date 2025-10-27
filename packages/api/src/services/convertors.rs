use backend::{AvailableGame, AvailableGameSource, Card, Game, PLAYER0, PLAYER1, Player, State};
use dto::{AvailableGameDTO, CardDTO, GameIdDTO, UserGameDTO};

pub fn game_to_user_game_dto(game: &Game, user_id: &backend::UserId) -> UserGameDTO {
    let user_is_host = || game.host() == user_id;
    let user_is_guest = || game.guest() == Some(user_id);

    let face_up = |card: &Card| CardDTO::FaceUp { cid: card.cid() };
    let _face_down = |_card: &Card| CardDTO::FaceDown;

    let view_for_user_opponent = |user: Player, opponent: Player| match game.state() {
        State::Starting(_) if game.guest().is_none() => UserGameDTO::new(game.name()),
        State::Starting(state) => {
            let user_cut =
                (!state.pending().waiting_on(user)).then_some(face_up(&state.cuts()[user]));
            let opponent_cut =
                (!state.pending().waiting_on(opponent)).then_some(face_up(&state.cuts()[opponent]));
            UserGameDTO::new(game.name())
                .with_user_cut(user_cut)
                .with_opponent_cut(opponent_cut)
        }
        State::Discarding(_state) => UserGameDTO::new("TODO: Discarding UserGameDTO"),
        State::Playing(_state) => UserGameDTO::new("TODO: Playing UserGameDTO"),
        State::ScoringPone(_state) => UserGameDTO::new("TODO: ScoringPone UserGameDTO"),
        State::ScoringDealer(_state) => UserGameDTO::new("TODO: ScoringDealer UserGameDTO"),
        State::ScoringCrib(_state) => UserGameDTO::new("TODO: ScoringCrib UserGameDTO"),
        State::Finished(_state) => UserGameDTO::new("TODO: Finished UserGameDTO"),
    };

    if user_is_host() {
        view_for_user_opponent(PLAYER0, PLAYER1)
    } else if user_is_guest() {
        view_for_user_opponent(PLAYER1, PLAYER0)
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
