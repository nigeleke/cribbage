use serde::{Deserialize, Serialize};

use crate::{
    display::format_vec,
    domain::{
        Card, Crib, Deck, Hands, HasCrib, HasDeck, HasHands, HasPending, HasRoles, HasScoreboard,
        Pending, Player, Roles, Scoreboard,
    },
};

/// Represents the game state during the *discarding* phase, where
/// players select cards to contribute to the crib.
///
/// This phase occurs after roles have been assigned and hands dealt,
/// but before the play phase begins. Each player removes cards from
/// their hand, and those cards are accumulated into the crib for the
/// upcoming scoring sequence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Discarding {
    scoreboard: Scoreboard,
    roles: Roles,
    hands: Hands,
    crib: Crib,
    deck: Deck,
    pending: Pending,
}

impl Discarding {
    /// Constructs a new `Discarding` state.
    ///
    /// The caller must ensure that the provided inputs are coherent and reflect
    /// a valid pre-discarding state.
    pub fn new(
        scoreboard: Scoreboard,
        roles: Roles,
        hands: Hands,
        crib: Crib,
        deck: Deck,
        pending: Pending,
    ) -> Self {
        Self {
            scoreboard,
            roles,
            hands,
            crib,
            deck,
            pending,
        }
    }

    /// Moves the specified cards from `player`’s hand into the crib.
    ///
    /// This method performs the following operations atomically:
    /// 1. Removes all `discards` from the player's hand.
    /// 2. Adds all `discards` to the crib.
    /// 3. Marks the player as having completed their discard action
    ///    via the pending-state mechanism.
    pub fn discard_cards_to_crib(&mut self, player: Player, discards: &[Card]) {
        self.hands[player].remove_all(discards);
        self.crib.add_all(discards);
        let _ = self.pending.acknowledge(player);
    }
}

impl HasScoreboard for Discarding {
    fn scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    fn scoreboard_mut(&mut self) -> &mut Scoreboard {
        &mut self.scoreboard
    }
}

impl HasRoles for Discarding {
    fn roles(&self) -> &Roles {
        &self.roles
    }

    fn roles_mut(&mut self) -> &mut Roles {
        &mut self.roles
    }
}

impl HasHands for Discarding {
    fn hands(&self) -> &Hands {
        &self.hands
    }

    fn hands_mut(&mut self) -> &mut Hands {
        &mut self.hands
    }
}

impl HasCrib for Discarding {
    fn crib(&self) -> &Crib {
        &self.crib
    }

    fn crib_mut(&mut self) -> &mut Crib {
        &mut self.crib
    }
}

impl HasDeck for Discarding {
    fn deck(&self) -> &Deck {
        &self.deck
    }

    fn deck_mut(&mut self) -> &mut Deck {
        &mut self.deck
    }
}

impl HasPending for Discarding {
    fn pending(&self) -> &Pending {
        &self.pending
    }

    fn pending_mut(&mut self) -> &mut Pending {
        &mut self.pending
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
