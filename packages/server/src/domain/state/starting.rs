use serde::{Deserialize, Serialize};

use crate::display::format_vec;
use crate::domain::{Cut, Cuts, Deck, Pending, Player, Roles};

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

    pub fn cut(&self, player: Player) -> Option<&Cut> {
        self.cuts[player].as_ref()
    }

    pub fn set_cut(&mut self, player: Player, cut: Cut) {
        self.cuts[player] = Some(cut);
        self.deck.remove(cut);
    }

    pub fn deck(&self) -> &Deck {
        &self.deck
    }

    pub fn pending(&self) -> &WaitingForCuts {
        &self.pending
    }

    pub fn set_acknowledged(&mut self, player: Player) {
        self.pending.acknowledge(player);
    }

    pub fn roles(&self) -> Option<Roles> {
        Roles::from_cuts(&self.cuts)
    }
}

impl Default for Starting {
    fn default() -> Self {
        let cuts = [None, None];
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
        let cuts = cuts
            .iter()
            .map(|c| c.map_or(String::from("--"), |c: Cut| c.to_string()))
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
