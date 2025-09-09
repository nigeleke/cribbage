use crate::{Error, Event, EventKind, Game, GameId, Player, ScoringPone, State, prettify};
use eventsourced::{Command, CommandEffect};
use eventsourced_ext::lift_effect;

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

impl Command<ScoringPone> for AcknowledgePoneScore {
    type Reply = bool;
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        _state: &ScoringPone,
    ) -> CommandEffect<ScoringPone, Self::Reply, Self::Error> {
        let player = self.player;

        CommandEffect::emit_and_reply(
            Event::new(*id, EventKind::PoneScoreAcknowledged { player }),
            move |s: &ScoringPone| {
                let mut pending = s.pending().clone();
                pending.acknowledge(player)
            },
        )
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
                lift_effect!(
                    scoring,
                    AcknowledgePoneScore::new(*id, player).handle_command(id, scoring)
                )
            }
            _ => CommandEffect::reject(Error::NotPermitted(prettify!(ScorePone))),
        }
    }
}
