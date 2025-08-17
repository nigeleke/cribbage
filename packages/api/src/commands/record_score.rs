use crate::{Error, Event, Game, GameId, Player, ScoreBreakdown};
use eventsourced::{Command, CommandEffect};

#[derive(Debug)]
pub struct RecordScore {
    game_id: GameId,
    player: Player,
    breakdown: ScoreBreakdown,
}

impl RecordScore {
    pub fn new(game_id: GameId, player: Player, breakdown: ScoreBreakdown) -> Self {
        Self {
            game_id,
            player,
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
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        panic!("boom");
        let RecordScore {
            game_id,
            player,
            breakdown,
        } = self;
        CommandEffect::emit(Event::ScoreRecorded {
            game_id,
            player,
            breakdown,
        })
    }
}
