use crate::{
    Deck, Discarding, Error, Event, Finished, Game, GameId, Pending, Player, ScoringCrib, State,
    constants::PLAYER_COUNT, prettify,
};
use eventsourced::{Command, CommandEffect};

#[derive(Debug)]
pub struct AcknowledgeCribScore {
    game_id: GameId,
    player: Player,
}

impl AcknowledgeCribScore {
    pub fn new(game_id: GameId, player: Player) -> Self {
        Self { game_id, player }
    }
}

impl Command<Game> for AcknowledgeCribScore {
    type Reply = bool;
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        let player = self.player;

        match state.state() {
            State::ScoringCrib(scoring) => {
                let (mut scoreboard, mut roles, hands, crib, cut, breakdown, mut pending) =
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
                        roles.swap();
                        let mut deck = Deck::shuffled_pack();
                        let hands = deck.deal(PLAYER_COUNT);
                        let hands = [hands[0].clone(), hands[1].clone()];
                        let pending = Pending::default();
                        let discarding =
                            Discarding::new(scoreboard, roles, hands, crib, deck, pending);
                        let state = State::Discarding(discarding);

                        CommandEffect::emit_and_reply(Event::state_updated(*id, state), move |_| {
                            proceeding
                        })
                    }
                } else {
                    let scoring =
                        ScoringCrib::new(scoreboard, roles, hands, crib, cut, breakdown, pending);
                    let state = State::ScoringCrib(scoring);
                    CommandEffect::emit_and_reply(Event::state_updated(*id, state), move |_| {
                        proceeding
                    })
                }
            }
            _ => CommandEffect::reject(Error::NotPermitted(prettify!(AcknowledgeCribScore))),
        }
    }
}
