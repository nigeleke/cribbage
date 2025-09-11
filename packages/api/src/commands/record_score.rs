use crate::{Error, Event, EventKind, Game, GameId, Player, ScoreBreakdown, ScorePhase};
use eventsourced::{Command, CommandEffect};

#[derive(Debug)]
pub struct RecordScore {
    game_id: GameId,
    player: Player,
    phase: ScorePhase,
    breakdown: ScoreBreakdown,
}

impl RecordScore {
    pub fn new(
        game_id: GameId,
        player: Player,
        phase: ScorePhase,
        breakdown: ScoreBreakdown,
    ) -> Self {
        Self {
            game_id,
            player,
            phase,
            breakdown,
        }
    }
}

impl Command<Game> for RecordScore {
    type Reply = ();
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        _state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        let player = self.player;
        let phase = self.phase;
        let breakdown = self.breakdown;

        CommandEffect::emit(Event::new(
            *id,
            EventKind::ScoreRecorded {
                player,
                phase,
                breakdown,
            },
        ))
    }
}
