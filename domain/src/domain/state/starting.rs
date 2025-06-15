use crate::{
    display::format_hashmap,
    domain::{Cuts, Deck, HasDeck, HasPlayers, Players},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Starting {
    cuts: Cuts,
    deck: Deck,
}

impl Starting {
    pub fn new(cuts: Cuts, deck: Deck) -> Self {
        Self { cuts, deck }
    }

    pub fn cuts(&self) -> &Cuts {
        &self.cuts
    }
}

impl HasPlayers for Starting {
    fn players(&self) -> Players {
        Players::from_iter(self.cuts.keys().copied())
    }
}

impl HasDeck for Starting {
    fn deck(&self) -> &Deck {
        &self.deck
    }
}

impl std::fmt::Display for Starting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { cuts, deck } = self;
        let cuts = format_hashmap(cuts);

        write!(
            f,
            r#"Starting(
    cuts: {cuts},
    deck: {deck}
)"#
        )
    }
}
