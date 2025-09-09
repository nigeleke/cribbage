use crate::{
    Card, Discarding, Error, Event, EventKind, Game, GameId, Player, State,
    constants::CARDS_DISCARDED_TO_CRIB, display::format_vec, prettify,
};
use eventsourced::{Command, CommandEffect};
use eventsourced_ext::lift_effect;

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

impl Command<Discarding> for DiscardCardsToCrib {
    type Reply = bool;
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Discarding,
    ) -> CommandEffect<Discarding, Self::Reply, Self::Error> {
        let Self {
            game_id: _,
            player,
            discards,
        } = self;

        let can_discard = state.pending().waiting_on(player);
        let valid_discard_count = discards.len() == CARDS_DISCARDED_TO_CRIB;
        let valid_discard_cards = state.hand(player).contains_all(&discards);
        let valid = can_discard && valid_discard_count && valid_discard_cards;

        if !valid {
            CommandEffect::reject(Error::InvalidDiscards(format_vec(&discards)))
        } else {
            CommandEffect::emit_and_reply(
                Event::new(*id, EventKind::CardsDiscardedToCrib { player, discards }),
                move |d: &Discarding| {
                    let mut pending = d.pending().clone();
                    pending.acknowledge(player)
                },
            )
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
            State::Discarding(discarding) => lift_effect!(
                discarding,
                DiscardCardsToCrib::new(*id, self.player, self.discards)
                    .handle_command(id, discarding)
            ),
            _ => CommandEffect::reject(Error::NotPermitted(prettify!(DiscardCardsToCrib))),
        }
    }
}
