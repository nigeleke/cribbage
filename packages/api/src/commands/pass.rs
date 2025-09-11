use crate::{Error, Event, EventKind, Game, GameId, Player, State, prettify};
use eventsourced::{Command, CommandEffect};

#[derive(Debug)]
pub struct Pass {
    game_id: GameId,
    player: Player,
}

impl Pass {
    pub fn new(game_id: GameId, player: Player) -> Self {
        Self { game_id, player }
    }
}

impl Command<Game> for Pass {
    type Reply = ();
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        match state.state() {
            State::Playing(playing) => {
                let player = self.player;

                if playing.play_state().next_to_play() != player {
                    CommandEffect::reject(Error::NotPlayersTurn(player))
                } else if !playing.play_state().legal_plays(player).is_empty() {
                    CommandEffect::reject(Error::InvalidPass)
                } else {
                    CommandEffect::emit(Event::new(*id, EventKind::Passed { player }))
                }
            }
            _ => CommandEffect::reject(Self::Error::NotPermitted(prettify!(CutCardAtStartOfPlay))),
        }
    }
}
