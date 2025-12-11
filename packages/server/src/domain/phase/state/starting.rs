use serde::{Deserialize, Serialize};

use crate::{
    display::format_vec,
    domain::{CutsForDeal, Deck, HasCutsForDeal, HasDeck, HasPending, Pending, Roles},
};

/// Represents the state of the game during the *starting* phase,
/// before the deal is assigned and initial hands are dealt.
///
/// `cuts_for_deal` entries will be `None` until a player has made a cut.
/// `pending` reflects player's acknowledgements of the cuts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Starting {
    cuts_for_deal: CutsForDeal,
    deck: Deck,
    pending: Pending,
}

impl Starting {
    /// Creates a new `Starting` state with the provided cut information,
    /// deck, and pending actions.
    ///
    /// The caller is responsible for ensuring that the inputs are valid for
    /// the beginning of the current starting state of the game.
    pub fn new(cuts_for_deal: CutsForDeal, deck: Deck, pending: Pending) -> Self {
        Self {
            cuts_for_deal,
            deck,
            pending,
        }
    }

    /// Computes and returns the player roles based on the completed cuts.
    ///
    /// If the cuts are incomplete or have the same face value then the
    /// function returns `None`.
    #[must_use]
    pub fn roles(&self) -> Option<Roles> {
        Roles::from_cuts(&self.cuts_for_deal)
    }
}

impl HasCutsForDeal for Starting {
    fn cuts_for_deal(&self) -> &CutsForDeal {
        &self.cuts_for_deal
    }

    fn cuts_for_deal_mut(&mut self) -> &mut CutsForDeal {
        &mut self.cuts_for_deal
    }
}

impl HasDeck for Starting {
    fn deck(&self) -> &Deck {
        &self.deck
    }

    fn deck_mut(&mut self) -> &mut Deck {
        &mut self.deck
    }
}

impl HasPending for Starting {
    fn pending(&self) -> &Pending {
        &self.pending
    }

    fn pending_mut(&mut self) -> &mut Pending {
        &mut self.pending
    }
}

impl Default for Starting {
    fn default() -> Self {
        let cuts = [None, None];
        let deck = Deck::shuffled_pack();
        let pending = Pending::default();
        Starting {
            cuts_for_deal: cuts,
            deck,
            pending,
        }
    }
}

impl std::fmt::Display for Starting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            cuts_for_deal: cuts,
            deck,
            pending,
        } = self;
        let cuts = cuts
            .iter()
            .map(|c| c.map_or(String::from("--"), |c| c.to_string()))
            .collect::<Vec<_>>();
        let cuts = format_vec(&cuts);

        write!(
            f,
            r#"Starting(
    cuts: {cuts}
    deck: {deck}
    pending: {pending}
)"#
        )
    }
}
