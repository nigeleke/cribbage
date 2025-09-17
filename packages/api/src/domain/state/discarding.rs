use crate::{
    Card, Crib, Dealer, Deck, Hand, Hands, Pending, Player, Pone, Roles, Scoreboard,
    display::format_vec,
};
use serde::{Deserialize, Serialize};

pub type WaitingForDiscards = Pending;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Discarding {
    scoreboard: Scoreboard,
    roles: Roles,
    hands: Hands,
    crib: Crib,
    deck: Deck,
    pending: WaitingForDiscards,
}

impl Discarding {
    #[rustfmt::skip]
    pub fn new(scoreboard: Scoreboard, roles: Roles, hands: Hands, crib: Crib, deck: Deck, pending: WaitingForDiscards) -> Self {
        Self { scoreboard, roles, hands, crib, deck, pending }
    }

    pub fn discard_cards_to_crib(&mut self, player: Player, discards: &[Card]) {
        self.hands[player].remove_all(discards);
        self.crib.add_all(discards);
        self.pending.acknowledge(player);
    }

    pub fn into_parts(self) -> (Scoreboard, Roles, Hands, Crib, Deck, WaitingForDiscards) {
        #[rustfmt::skip]
        let Self { scoreboard, roles, hands, crib, deck, pending } = self;
        (scoreboard, roles, hands, crib, deck, pending)
    }

    pub fn scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    pub fn dealer(&self) -> &Dealer {
        self.roles.dealer()
    }

    pub fn pone(&self) -> &Pone {
        self.roles.pone()
    }

    pub fn hand(&self, player: Player) -> &Hand {
        &self.hands[player]
    }

    #[cfg(test)]
    pub fn crib(&self) -> &Crib {
        &self.crib
    }

    pub fn deck(&self) -> &Deck {
        &self.deck
    }

    pub fn pending(&self) -> &WaitingForDiscards {
        &self.pending
    }
}

impl std::fmt::Display for Discarding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[rustfmt::skip]
        let Self { scoreboard, roles, hands, crib, deck, pending } = self;
        let hands = format_vec(hands);

        write!(
            f,
            r#"Discarding(
    scoreboard: {scoreboard}
    roles: {roles}
    hands: {hands}
    crib: {crib}
    deck: {deck}
    pending: {pending}
)"#
        )
    }
}
