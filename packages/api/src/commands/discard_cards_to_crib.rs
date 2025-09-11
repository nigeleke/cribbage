use crate::{
    Card, Error, Event, EventKind, Game, GameId, Player, State, constants::CARDS_DISCARDED_TO_CRIB,
    display::format_vec, prettify,
};
use eventsourced::{Command, CommandEffect};

#[derive(Debug)]
pub struct DiscardCardsToCrib {
    game_id: GameId,
    player: Player,
    discards: Vec<Card>,
}

impl DiscardCardsToCrib {
    pub fn new(game_id: GameId, player: Player, discards: Vec<Card>) -> Self {
        Self {
            game_id,
            player,
            discards,
        }
    }
}

impl Command<Game> for DiscardCardsToCrib {
    type Reply = bool;
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        match state.state() {
            State::Discarding(discarding) => {
                let Self {
                    game_id: _,
                    player,
                    discards,
                } = self;

                let can_discard = discarding.pending().waiting_on(player);
                let valid_discard_count = discards.len() == CARDS_DISCARDED_TO_CRIB;
                let valid_discard_cards = discarding.hand(player).contains_all(&discards);
                let valid = can_discard && valid_discard_count && valid_discard_cards;

                let mut pending = discarding.pending().clone();
                let proceed = pending.acknowledge(player);

                if !valid {
                    CommandEffect::reject(Error::InvalidDiscards(format_vec(&discards)))
                } else {
                    CommandEffect::emit_and_reply(
                        Event::new(*id, EventKind::CardsDiscardedToCrib { player, discards }),
                        move |_| proceed,
                    )
                }
            }
            _ => CommandEffect::reject(Error::NotPermitted(prettify!(DiscardCardsToCrib))),
        }
    }
}
