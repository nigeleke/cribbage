use crate::{
    display::format_hashmap,
    domain::{Cuts, Deck, HasCuts, HasDeck, HasPlayers, Players},
};

#[derive(Debug)]
pub struct Starting {
    cuts: Cuts,
    deck: Deck,
}

impl Starting {
    pub const fn new(cuts: Cuts, deck: Deck) -> Self {
        Self { cuts, deck }
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

impl HasCuts for Starting {
    fn cuts(&self) -> &Cuts {
        &self.cuts
    }
}

impl std::fmt::Display for Starting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Starting(cuts: {}, deck: {})",
            format_hashmap(&self.cuts),
            self.deck
        )
    }
}
