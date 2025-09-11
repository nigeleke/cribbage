use crate::{Error, Event, EventKind, Game, GameId, Player, State, prettify};
use eventsourced::{Command, CommandEffect};

#[derive(Debug)]
pub struct AcknowledgePoneScore {
    game_id: GameId,
    player: Player,
}

impl AcknowledgePoneScore {
    pub fn new(game_id: GameId, player: Player) -> Self {
        Self { game_id, player }
    }
}

impl Command<Game> for AcknowledgePoneScore {
    type Reply = bool;
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        let player = self.player;
        match state.state() {
            State::ScoringPone(scoring) => {
                let mut pending = scoring.pending().clone();
                let proceed = pending.acknowledge(player);
                CommandEffect::emit_and_reply(
                    Event::new(*id, EventKind::PoneHandScoreAcknowledged { player }),
                    move |_| proceed,
                )
            }
            _ => CommandEffect::reject(Error::NotPermitted(prettify!(ScorePone))),
        }
    }
}
