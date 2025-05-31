use crate::{
    display::format_hashmap,
    domain::{Cuts, Deck, HasDeck, HasPlayers, Players},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Starting {
    pub cuts: Cuts,
    pub deck: Deck,
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
        write!(
            f,
            r#"Starting(
    cuts: {},
    deck: {}
)"#,
            format_hashmap(&self.cuts),
            self.deck
        )
    }
}
