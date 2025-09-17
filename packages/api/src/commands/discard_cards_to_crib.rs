use crate::{
    Card, Discarding, Error, Event, Finished, Game, GameId, PLAYER0, PLAYER1, PlayState, Player,
    Playing, ScoreBreakdown, State, constants::CARDS_DISCARDED_TO_CRIB, display::format_vec,
    prettify,
};
use eventsourced::{Command, CommandEffect};

#[derive(Debug)]
pub struct DiscardCardsToCrib {
    game_id: GameId,
    player: Player,
    discards: Vec<Card>,
}

impl DiscardCardsToCrib {
    pub fn new(game_id: GameId, player: Player, discards: &[Card]) -> Self {
        Self {
            game_id,
            player,
            discards: Vec::from(discards),
        }
    }
}

impl Command<Game> for DiscardCardsToCrib {
    type Reply = bool;
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        game: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        match game.state() {
            State::Discarding(discarding) => {
                let player = self.player;
                let discards = self.discards;

                let (mut scoreboard, roles, mut hands, mut crib, mut deck, mut pending) =
                    discarding.clone().into_parts();

                let can_discard = pending.waiting_on(player);
                let valid_discard_count = discards.len() == CARDS_DISCARDED_TO_CRIB;
                let valid_discard_cards = hands[player].contains_all(&discards);
                let valid = can_discard && valid_discard_count && valid_discard_cards;

                if !valid {
                    return CommandEffect::reject(Error::InvalidDiscards(format_vec(&discards)));
                }

                hands[player].remove_all(&discards);
                crib.add_all(&discards);
                let proceeding = pending.acknowledge(player);

                if !proceeding {
                    let discarding = Discarding::new(scoreboard, roles, hands, crib, deck, pending);
                    let state = State::Discarding(discarding);
                    CommandEffect::emit_and_reply(Event::state_updated(*id, state), move |_| {
                        proceeding
                    })
                } else {
                    let cut = deck.cut();

                    let breakdown = ScoreBreakdown::his_heels(cut);
                    scoreboard.peg(roles.dealer().player(), &breakdown);

                    if let Some(winner) = scoreboard.winner() {
                        let finished = Finished::new(winner, scoreboard, roles, hands, crib, cut);
                        let state = State::Finished(finished);
                        CommandEffect::emit_and_reply(Event::state_updated(*id, state), |_| true)
                    } else {
                        let play_state = PlayState::new(roles.pone().player())
                            .with_pending_plays(PLAYER0, hands[PLAYER0].as_ref())
                            .with_pending_plays(PLAYER1, hands[PLAYER1].as_ref());
                        let playing = Playing::new(scoreboard, roles, hands, play_state, crib, cut);
                        let state = State::Playing(playing);
                        CommandEffect::emit_and_reply(Event::state_updated(*id, state), move |_| {
                            proceeding
                        })
                    }
                }
            }
            _ => CommandEffect::reject(Error::NotPermitted(prettify!(DiscardCardsToCrib))),
        }
    }
}
