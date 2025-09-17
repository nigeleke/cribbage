use crate::{
    Error, Event, Finished, Game, GameId, Pending, Player, ScoreBreakdown, ScoringCrib,
    ScoringDealer, State, prettify,
};
use eventsourced::{Command, CommandEffect, EventSourcedExt};

#[derive(Debug)]
pub struct AcknowledgeDealerScore {
    game_id: GameId,
    player: Player,
}

impl AcknowledgeDealerScore {
    pub fn new(game_id: GameId, player: Player) -> Self {
        Self { game_id, player }
    }
}

impl Command<Game> for AcknowledgeDealerScore {
    type Reply = bool;
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        let player = self.player;

        match state.state() {
            State::ScoringDealer(scoring) => {
                let (mut scoreboard, roles, hands, crib, cut, breakdown, mut pending) =
                    scoring.clone().into_parts();

                let proceeding = pending.acknowledge(player);

                if proceeding {
                    scoreboard.peg(roles.dealer().player(), &breakdown);
                    if let Some(winner) = scoreboard.winner() {
                        let finished = Finished::new(winner, scoreboard, roles, hands, crib, cut);
                        let state = State::Finished(finished);
                        CommandEffect::emit_and_reply(Event::state_updated(*id, state), move |_| {
                            proceeding
                        })
                    } else {
                        let pending = Pending::default();
                        let breakdown = ScoreBreakdown::crib(&crib, cut);

                        let scoring = ScoringCrib::new(
                            scoreboard, roles, hands, crib, cut, breakdown, pending,
                        );
                        let state = State::ScoringCrib(scoring);

                        CommandEffect::emit_and_reply(Event::state_updated(*id, state), move |_| {
                            proceeding
                        })
                    }
                } else {
                    let scoring =
                        ScoringDealer::new(scoreboard, roles, hands, crib, cut, breakdown, pending);
                    let state = State::ScoringDealer(scoring);
                    CommandEffect::emit_and_reply(Event::state_updated(*id, state), move |_| {
                        proceeding
                    })
                }
            }
            _ => CommandEffect::reject(Error::NotPermitted(prettify!(AcknowledgeDealerScore))),
        }
    }
}
