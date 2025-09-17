use crate::{Cut, Cuts, Deck, Pending, Player, display::format_vec};
use serde::{Deserialize, Serialize};

pub type WaitingForCuts = Pending;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Starting {
    cuts: Cuts,
    deck: Deck,
    pending: WaitingForCuts,
}

impl Starting {
    pub fn new(cuts: Cuts, deck: Deck, pending: WaitingForCuts) -> Self {
        Self {
            cuts,
            deck,
            pending,
        }
    }

    pub fn into_parts(self) -> (Cuts, Deck, WaitingForCuts) {
        let Self {
            cuts,
            deck,
            pending,
        } = self;
        (cuts, deck, pending)
    }

    pub fn cuts(&self) -> &Cuts {
        &self.cuts
    }

    pub fn deck(&self) -> &Deck {
        &self.deck
    }

    pub fn pending(&self) -> &WaitingForCuts {
        &self.pending
    }
}

impl Default for Starting {
    fn default() -> Self {
        let cuts = [Cut::placeholder(), Cut::placeholder()];
        let deck = Deck::shuffled_pack();
        let pending = WaitingForCuts::default();
        Starting {
            cuts,
            deck,
            pending,
        }
    }
}

impl std::fmt::Display for Starting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            cuts,
            deck,
            pending,
        } = self;
        let cuts = format_vec(cuts);

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
