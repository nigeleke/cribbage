use eventsourced::{Command, CommandEffect};

use crate::constants::PLAYER_COUNT;
use crate::{
    Crib, Deck, Discarding, Event, Finished, Game, GameError, GameId, Hands, Pending, Player,
    Roles, ScoreBreakdown, ScoringCrib, ScoringDealer, ScoringPone, State, prettify,
};

macro_rules! acknowledge_score {
    ($cmd:ident, $scoring_type:ty, $state_variant:ident, $peg_player:expr, $next:expr) => {
        #[derive(Debug)]
        pub struct $cmd {
            game_id: GameId,
            player: Player,
        }

        impl $cmd {
            pub fn new(game_id: GameId, player: Player) -> Self {
                Self { game_id, player }
            }
        }

        impl Command<Game> for $cmd {
            type Reply = bool;
            type Error = GameError;

            fn handle_command(
                self,
                id: &GameId,
                state: &Game,
            ) -> CommandEffect<Game, Self::Reply, Self::Error> {
                match state.state() {
                    State::$state_variant(scoring) => {
                        let (mut scoreboard, roles, hands, crib, cut, breakdown, mut pending) =
                            scoring.clone().into_parts();

                        let proceeding = pending.acknowledge(self.player);

                        if proceeding {
                            scoreboard.peg($peg_player(&roles), &breakdown);
                            if let Some(winner) = scoreboard.winner() {
                                let finished =
                                    Finished::new(winner, scoreboard, roles, hands, crib, cut);
                                let state = State::Finished(finished);
                                CommandEffect::emit_and_reply(
                                    Event::state_updated(*id, state),
                                    move |_| proceeding,
                                )
                            } else {
                                $next(*id, scoreboard, roles, hands, crib, cut, proceeding)
                            }
                        } else {
                            let scoring = <$scoring_type>::new(
                                scoreboard, roles, hands, crib, cut, breakdown, pending,
                            );
                            let state = State::$state_variant(scoring);
                            CommandEffect::emit_and_reply(
                                Event::state_updated(*id, state),
                                move |_| proceeding,
                            )
                        }
                    }
                    _ => CommandEffect::reject(GameError::NotPermitted(prettify!($cmd))),
                }
            }
        }
    };
}

acknowledge_score!(
    AcknowledgePoneScore,
    ScoringPone,
    ScoringPone,
    |roles: &Roles| roles.pone().player(),
    |id: GameId, scoreboard, roles: Roles, hands: Hands, crib, cut, proceeding| {
        let pending = Pending::default();
        let breakdown = ScoreBreakdown::hand(&hands[roles.dealer()], cut);
        let scoring = ScoringDealer::new(scoreboard, roles, hands, crib, cut, breakdown, pending);
        let state = State::ScoringDealer(scoring);
        CommandEffect::emit_and_reply(Event::state_updated(id, state), move |_| proceeding)
    }
);

acknowledge_score!(
    AcknowledgeDealerScore,
    ScoringDealer,
    ScoringDealer,
    |roles: &Roles| roles.dealer().player(),
    |id: GameId, scoreboard, roles, hands, crib, cut, proceeding| {
        let pending = Pending::default();
        let breakdown = ScoreBreakdown::crib(&crib, cut);
        let scoring = ScoringCrib::new(scoreboard, roles, hands, crib, cut, breakdown, pending);
        let state = State::ScoringCrib(scoring);
        CommandEffect::emit_and_reply(Event::state_updated(id, state), move |_| proceeding)
    }
);

acknowledge_score!(
    AcknowledgeCribScore,
    ScoringCrib,
    ScoringCrib,
    |roles: &Roles| roles.dealer().player(),
    |id: GameId, scoreboard, mut roles: Roles, _hands, _crib, _cut, proceeding| {
        roles.swap();
        let mut deck = Deck::shuffled_pack();
        let hands = deck.deal(PLAYER_COUNT);
        let hands = [hands[0].clone(), hands[1].clone()];
        let crib = Crib::default();
        let pending = Pending::default();
        let discarding = Discarding::new(scoreboard, roles, hands, crib, deck, pending);
        let state = State::Discarding(discarding);
        CommandEffect::emit_and_reply(Event::state_updated(id, state), move |_| proceeding)
    }
);
