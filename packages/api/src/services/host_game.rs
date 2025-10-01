use dioxus::prelude::*;
use dto::{GameIdDTO, UserIdDTO};
use eventsourced::{Command, CommandEffect};

use crate::domain::*;

pub async fn host_game(user_id: UserIdDTO) -> Result<GameIdDTO, ServerFnError> {
    let user_id = UserId::from(user_id);
    let game = Game::default();
    let game_id = game.id();

    let command = HostGame::new(user_id);
    let game_id = match command.handle_command(game_id, &game) {
        CommandEffect::EmitAndReply(_, f) => Ok(f(&game).0),
        CommandEffect::Reply(reply) => Ok(reply.0),
        CommandEffect::Reject(error) => Err(ServerFnError::from(error)),
    }?;

    Ok(GameIdDTO::from(game_id))
}
