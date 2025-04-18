use crate::constants::*;
use crate::display::format_hashmap;
use crate::domain::{
    Card, Crib, Deck, Hands, HasCrib, HasDeck, HasHands, HasPlayers, HasRoles, HasScores, Player,
    Players, Roles, Scores,
};

#[derive(Debug)]
pub struct Discarding {
    scores: Scores,
    roles: Roles,
    hands: Hands,
    crib: Crib,
    deck: Deck,
}

pub enum DiscardingState {
    StillDiscarding,
    ReadyToCut,
}

impl Discarding {
    pub fn new(scores: Scores, roles: Roles, hands: Hands, crib: Crib, deck: Deck) -> Self {
        Self {
            scores,
            roles,
            hands,
            crib,
            deck,
        }
    }

    pub fn into_parts(self) -> (Scores, Roles, Hands, Crib, Deck) {
        let Self {
            scores,
            roles,
            hands,
            crib,
            deck,
        } = self;
        (scores, roles, hands, crib, deck)
    }

    pub fn discard(&mut self, player: Player, discards: &[Card]) -> DiscardingState {
        let hand = self
            .hands
            .get_mut(&player)
            .expect(stringify!(Discarding::discard));
        hand.remove_all(discards);

        let crib = &mut self.crib;
        crib.add(discards);

        if crib.len() == CARDS_REQUIRED_IN_CRIB {
            DiscardingState::ReadyToCut
        } else {
            DiscardingState::StillDiscarding
        }
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

    fn scores_mut(&mut self) -> &mut Scores {
        &mut self.scores
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

impl std::fmt::Display for Discarding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Discarding(scores: {}, roles: {}, hands: {}, crib: {}, deck: {})",
            self.scores,
            self.roles,
            format_hashmap(&self.hands),
            self.crib,
            self.deck
        )
    }
}
