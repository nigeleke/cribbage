use crate::{
    display::format_hashmap,
    domain::{
        Crib, Deck, Hands, HasCrib, HasDeck, HasHands, HasPlayers, HasRoles, HasScores, Players,
        Roles, Scores,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Discarding {
    scores: Scores,
    roles: Roles,
    hands: Hands,
    crib: Crib,
    deck: Deck,
}

impl Discarding {
    #[rustfmt::skip]
    pub fn new(scores: Scores, roles: Roles, hands: Hands, crib: Crib, deck: Deck) -> Self {
        Self { scores, roles, hands, crib, deck }
    }

    pub fn into_parts(self) -> (Scores, Roles, Hands, Crib, Deck) {
        #[rustfmt::skip]
        let Self { scores, roles, hands, crib, deck } = self;
        (scores, roles, hands, crib, deck)
    }
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
        #[rustfmt::skip]
        let Self { scores, roles, hands, crib, deck } = self;
        let hands = format_hashmap(hands);

        write!(
            f,
            r#"Discarding(
    scores: {scores},
    roles: {roles},
    hands: {hands},
    crib: {crib},
    deck: {deck}
)"#
        )
    }
}
