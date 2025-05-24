use serde::{Deserialize, Serialize};

use crate::{
    display::format_hashmap,
    domain::{
        Crib, Deck, Hands, HasCrib, HasDeck, HasHands, HasPlayers, HasRoles, HasScores, Players,
        Roles, Scores,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Discarding {
    pub scores: Scores,
    pub roles: Roles,
    pub hands: Hands,
    pub crib: Crib,
    pub deck: Deck,
}

impl HasPlayers for Discarding {
    fn players(&self) -> Players {
        Players::from_iter(self.hands.keys().copied())
    }
}

impl HasScores for Discarding {
    fn scores(&self) -> &Scores {
        &self.scores
    }
}

impl HasRoles for Discarding {
    fn roles(&self) -> &Roles {
        &self.roles
    }
}

impl HasHands for Discarding {
    fn hands(&self) -> &Hands {
        &self.hands
    }
}

impl HasCrib for Discarding {
    fn crib(&self) -> &Crib {
        &self.crib
    }
}

impl HasDeck for Discarding {
    fn deck(&self) -> &Deck {
        &self.deck
    }
}

impl std::fmt::Display for Discarding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Discarding(
    scores: {},
    roles: {},
    hands: {},
    crib: {},
    deck: {}
)"#,
            self.scores,
            self.roles,
            format_hashmap(&self.hands),
            self.crib,
            self.deck
        )
    }
}
